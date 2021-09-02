//! Firefly Algorithm.
//!
//! <https://en.wikipedia.org/wiki/Firefly_algorithm>
use crate::{utility::*, *};

setting! {
    /// Firefly Algorithm settings.
    pub struct Fa {
        @base,
        @pop_num = 80,
        /// Alpha factor.
        alpha: f64 = 0.05,
        /// Minimum beta factor.
        beta_min: f64 = 0.2,
        /// Gamma factor.
        gamma: f64 = 1.,
    }
}

impl Setting for Fa {
    type Algorithm = Method;

    fn base(&self) -> &BasicSetting {
        &self.base
    }

    fn create(self) -> Self::Algorithm {
        Method {
            alpha: self.alpha,
            beta_min: self.beta_min,
            gamma: self.gamma,
            beta0: 1.,
        }
    }
}

/// Firefly Algorithm type.
pub struct Method {
    alpha: f64,
    beta_min: f64,
    gamma: f64,
    beta0: f64,
}

impl Method {
    fn move_fireflies<F: ObjFunc>(&mut self, ctx: &mut Context<F>) {
        for (i, j) in product(0..ctx.pop_num, 0..ctx.pop_num) {
            if ctx.fitness[i] <= ctx.fitness[j] {
                continue;
            }
            let mut tmp = Array1::zeros(ctx.dim);
            let pool_j = if i == j {
                ctx.best.view()
            } else {
                ctx.pool.slice(s![j, ..])
            };
            let r = {
                let mut dist = 0.;
                for s in 0..ctx.dim {
                    let diff = ctx.pool[[i, s]] - pool_j[s];
                    dist += diff * diff;
                }
                dist
            };
            let beta = (self.beta0 - self.beta_min) * (-self.gamma * r).exp() + self.beta_min;
            for s in 0..ctx.dim {
                let v = ctx.pool[[i, s]]
                    + beta * (pool_j[s] - ctx.pool[[i, s]])
                    + self.alpha * (ctx.ub(s) - ctx.lb(s)) * rand_float(-0.5, 0.5);
                tmp[s] = ctx.check(s, v);
            }
            let tmp_f = ctx.func.fitness(tmp.as_slice().unwrap(), &ctx.report);
            if tmp_f < ctx.fitness[i] {
                ctx.assign_from(i, tmp_f, &tmp);
            }
        }
    }
}

impl Algorithm for Method {
    #[inline(always)]
    fn generation<F: ObjFunc>(&mut self, ctx: &mut Context<F>) {
        self.move_fireflies(ctx);
        ctx.find_best();
    }
}
