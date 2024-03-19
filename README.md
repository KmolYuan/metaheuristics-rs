# metaheuristics-nature

[![dependency status](https://deps.rs/repo/github/KmolYuan/metaheuristics-nature-rs/status.svg)](https://deps.rs/crate/metaheuristics-nature/)
[![documentation](https://docs.rs/metaheuristics-nature/badge.svg)](https://docs.rs/metaheuristics-nature)

A collection of nature-inspired metaheuristic algorithms. This crate provides objective function trait, well-known methods, and tool functions let you implement your own searching method.

This crate implemented following algorithms:

+ Real-coded Genetic Algorithm (RGA)
+ Differential Evolution (DE)
+ Particle Swarm Optimization (PSO)
+ Firefly Algorithm (FA)
+ Teaching-Learning Based Optimization (TLBO)
+ Support Pareto front for Multi-Objective Optimization (MOO)

Each algorithm gives same API and default parameters to help you test different implementation. For example, you can
test another algorithm by simply replacing `Rga` to `De`.

```rust
use metaheuristics_nature::{Rga, Solver};

let mut report = Vec::with_capacity(20);

// Build and run the solver
let s = Solver::build(Rga::default(), MyFunc::new())
    .seed(0)
    .task(|ctx| ctx.gen == 20)
    .callback(|ctx| report.push(ctx.best.get_eval()))
    .solve()
    .unwrap();
// Get the optimized XY value of your function
let (xs, p) = s.as_best();
// If `p` is a `Product` type wrapped with the fitness value
let err = p.ys();
let result = p.as_result();
// Get the history reports
let y2 = &report[2];
```

### What kinds of problems can be solved?

If your problem can be simulated and evaluated, the optimization method is the efficient way to find the best design! 🚀

Assuming that your simulation can be done with a function `f`, by inputting the parameters `X` and the evaluation value `y`, then the optimization method will try to adjust `X={x0, x1, ...}` to obtain the smallest `y`. Their relationship can be written as `f(X) = y`.

The number of the parameters `X` is called "dimension", imagine `X` is the coordinate in the multi-dimension, and `y` is the weight of the "point". If the dimension becomes higher, the problem will be more difficult to search.

The "metaheuristic" algorithms use multiple points to search for the minimum value, which detects the local gradient, across the most feasible solutions and keeps away from the local optimum. Even with an unknown gradient or feasible region.

Please see the API documentation for more information.

### Gradient-based Methods

For more simple functions, for example, if the 1st derivative function is known, the gradient-based methods are recommended to use for the fastest speed. Such as [OSQP](https://osqp.org/).
