use crate::prelude::*;
use alloc::{boxed::Box, vec::Vec};

type PoolFunc<'a> = Box<dyn Fn(usize, core::ops::RangeInclusive<f64>, &mut Rng) -> f64 + 'a>;

/// Initial pool generating options.
///
/// Use [`SolverBuilder::init_pool()`] to set this option.
pub enum Pool<'a, F: ObjFunc> {
    /// A ready-made pool and its fitness values.
    Ready {
        /// Pool
        pool: Vec<Vec<f64>>,
        /// Fitness values
        pool_y: Vec<F::Ys>,
    },
    /// Generate the pool uniformly with a filter function to check the
    /// validity.
    ///
    /// This filter function returns true if the design variables are valid.
    #[allow(clippy::type_complexity)]
    UniformBy(Box<dyn Fn(&[f64]) -> bool + 'a>),
    /// Generate the pool with a specific function.
    ///
    /// The function signature is `fn(s, min..max, &rng) -> value`
    /// + `s` is the index of the variable
    /// + `min..max` is the range of the variable
    /// + `rng` is the random number generator
    ///
    /// Two examples are [`uniform_pool()`] and [`gaussian_pool()`].
    ///
    /// ```
    /// use metaheuristics_nature::{gaussian_pool, Pool, Rga, Solver};
    /// # use metaheuristics_nature::tests::TestObj as MyFunc;
    ///
    /// let pool = Pool::Func(Box::new(gaussian_pool(&[0.; 4], &[1.; 4])));
    /// let s = Solver::build(Rga::default(), MyFunc::new())
    ///     .seed(0)
    ///     .task(|ctx| ctx.gen == 20)
    ///     .init_pool(pool)
    ///     .solve();
    /// ```
    Func(PoolFunc<'a>),
}

/// Collect configuration and build the solver.
///
/// This type is created by [`Solver::build()`] method.
#[allow(clippy::type_complexity)]
#[must_use = "solver builder do nothing unless call the \"solve\" method"]
pub struct SolverBuilder<'a, F: ObjFunc> {
    func: F,
    pop_num: usize,
    pareto_limit: usize,
    seed: SeedOpt,
    algorithm: Box<dyn Algorithm<F>>,
    pool: Pool<'a, F>,
    task: Box<dyn Fn(&Ctx<F>) -> bool + Send + 'a>,
    callback: Box<dyn FnMut(&mut Ctx<F>) + Send + 'a>,
}

impl<'a, F: ObjFunc> SolverBuilder<'a, F> {
    impl_builders! {
        /// Population number.
        ///
        /// # Default
        ///
        /// If not changed by the algorithm setting, the default number is 200.
        fn pop_num(usize)

    }

    /// Pareto front limit.
    ///
    /// It is not working for single-objective optimization.
    ///
    /// ```
    /// use metaheuristics_nature::{Rga, Solver};
    /// # use metaheuristics_nature::tests::TestMO as MyFunc;
    ///
    /// let s = Solver::build(Rga::default(), MyFunc::new())
    ///     .seed(0)
    ///     .task(|ctx| ctx.gen == 20)
    ///     .pareto_limit(10)
    ///     .solve();
    /// ```
    ///
    /// # Default
    ///
    /// If not changed by the algorithm setting, the default number is 20.
    pub fn pareto_limit(self, pareto_limit: usize) -> Self
    where
        F::Ys: Fitness<Best<F::Ys> = Pareto<F::Ys>>,
    {
        Self { pareto_limit, ..self }
    }

    /// Set the random seed to get a determined result.
    ///
    /// # Default
    ///
    /// By default, the random seed is auto-decided.
    pub fn seed(self, seed: impl Into<SeedOpt>) -> Self {
        Self { seed: seed.into(), ..self }
    }

    /// Initialize the pool with the pool option.
    ///
    /// # Default
    ///
    /// By default, the pool is generated by the uniform distribution
    /// [`uniform_pool()`].
    pub fn init_pool(self, pool: Pool<'a, F>) -> Self {
        Self { pool, ..self }
    }

    /// Termination condition.
    ///
    /// The task function will be check each iteration, breaks if the return is
    /// true.
    ///
    /// ```
    /// use metaheuristics_nature::{Rga, Solver};
    /// # use metaheuristics_nature::tests::TestObj as MyFunc;
    ///
    /// let s = Solver::build(Rga::default(), MyFunc::new())
    ///     .seed(0)
    ///     .task(|ctx| ctx.gen == 20)
    ///     .solve();
    /// ```
    ///
    /// # Default
    ///
    /// By default, the algorithm will iterate 200 generation.
    pub fn task<'b, C>(self, task: C) -> SolverBuilder<'b, F>
    where
        'a: 'b,
        C: Fn(&Ctx<F>) -> bool + Send + 'b,
    {
        SolverBuilder { task: Box::new(task), ..self }
    }

    /// Set callback function.
    ///
    /// Callback function allows to change an outer mutable variable in each
    /// iteration.
    ///
    /// ```
    /// use metaheuristics_nature::{Rga, Solver};
    /// # use metaheuristics_nature::tests::TestObj as MyFunc;
    ///
    /// let mut report = Vec::with_capacity(20);
    /// let s = Solver::build(Rga::default(), MyFunc::new())
    ///     .seed(0)
    ///     .task(|ctx| ctx.gen == 20)
    ///     .callback(|ctx| report.push(ctx.best.get_eval()))
    ///     .solve();
    /// ```
    ///
    /// # Default
    ///
    /// By default, this function does nothing.
    pub fn callback<'b, C>(self, callback: C) -> SolverBuilder<'b, F>
    where
        'a: 'b,
        C: FnMut(&mut Ctx<F>) + Send + 'b,
    {
        SolverBuilder { callback: Box::new(callback), ..self }
    }

    /// Create the task and run the algorithm, which may takes a lot of time.
    ///
    /// Generation `ctx.gen` is start from 1, initialized at 0.
    ///
    /// # Panics
    ///
    /// Panics when the boundary check failed.
    pub fn solve(self) -> Solver<F> {
        let Self {
            func,
            pop_num,
            pareto_limit,
            seed,
            mut algorithm,
            pool,
            task,
            mut callback,
        } = self;
        assert!(
            func.bound().iter().all(|[lb, ub]| lb <= ub),
            "Lower bound should be less than upper bound"
        );
        let mut rng = Rng::new(seed);
        let mut ctx = match pool {
            Pool::Ready { pool, pool_y } => {
                let dim = func.dim();
                assert!(pool.len() == pop_num, "Pool size mismatched");
                assert!(pool[0].len() == dim, "Pool dimension mismatched");
                Ctx::from_parts(func, pareto_limit, pool, pool_y)
            }
            Pool::UniformBy(filter) => {
                let dim = func.dim();
                let mut pool = Vec::with_capacity(pop_num);
                let rand_f = uniform_pool();
                while pool.len() < pop_num {
                    let x = (0..dim)
                        .map(|s| rand_f(s, func.bound_range(s), &mut rng))
                        .collect::<Vec<_>>();
                    if filter(&x) {
                        pool.push(x);
                    }
                }
                Ctx::from_pool(func, pareto_limit, pool)
            }
            Pool::Func(f) => {
                let dim = func.dim();
                let pool = (0..pop_num)
                    .map(|_| {
                        (0..dim)
                            .map(|s| f(s, func.bound_range(s), &mut rng))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                Ctx::from_pool(func, pareto_limit, pool)
            }
        };
        algorithm.init(&mut ctx, &mut rng);
        loop {
            callback(&mut ctx);
            if task(&ctx) {
                break;
            }
            ctx.gen += 1;
            algorithm.generation(&mut ctx, &mut rng);
        }
        Solver::new(ctx, rng.seed())
    }
}

impl<F: ObjFunc> Solver<F> {
    /// Start to build a solver. Take a setting and setup the configurations.
    ///
    /// Please check [`SolverBuilder`] type, it will help you choose your
    /// configuration.
    ///
    /// If all things are well-setup, call [`SolverBuilder::solve()`].
    ///
    /// The default value of each option can be found in their document.
    pub fn build<S>(setting: S, func: F) -> SolverBuilder<'static, F>
    where
        S: Setting,
    {
        SolverBuilder {
            func,
            pop_num: S::default_pop(),
            pareto_limit: 20,
            seed: SeedOpt::None,
            algorithm: Box::new(setting.algorithm()),
            pool: Pool::Func(Box::new(uniform_pool())),
            task: Box::new(|ctx| ctx.gen >= 200),
            callback: Box::new(|_| ()),
        }
    }
}

/// A function generates a uniform pool.
///
/// See also [`gaussian_pool()`], [`Pool::Func`],
/// [`SolverBuilder::init_pool()`].
pub fn uniform_pool() -> PoolFunc<'static> {
    Box::new(move |_, range, rng| rng.range(range))
}

/// A function generates a Gaussian pool.
///
/// Where `mean` is the mean value, `std` is the standard deviation.
///
/// See also [`uniform_pool()`], [`Pool::Func`], [`SolverBuilder::init_pool()`].
///
/// # Panics
///
/// Panic when the lengths of `mean` and `std` are not the same.
pub fn gaussian_pool<'a>(mean: &'a [f64], std: &'a [f64]) -> PoolFunc<'a> {
    assert_eq!(mean.len(), std.len());
    Box::new(move |s, _, rng| rng.normal(mean[s], std[s]))
}
