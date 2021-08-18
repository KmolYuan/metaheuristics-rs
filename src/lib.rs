//! A collection of nature-inspired metaheuristic algorithms.
//! ```
//! use metaheuristics_nature::{Report, RgaSetting, Solver, Task};
//! # use metaheuristics_nature::{ObjFunc, Array1, AsArray};
//! # struct MyFunc([f64; 3], [f64; 3]);
//! # impl MyFunc {
//! #     fn new() -> Self { Self([0.; 3], [50.; 3]) }
//! # }
//! # impl ObjFunc for MyFunc {
//! #     type Result = f64;
//! #     fn fitness<'a, A>(&self, v: A, _: &Report) -> f64
//! #     where
//! #         A: AsArray<'a, f64>,
//! #     {
//! #         let v = v.into();
//! #         v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
//! #     }
//! #     fn result<'a, V>(&self, v: V) -> Self::Result
//! #     where
//! #         V: AsArray<'a, f64>
//! #     {
//! #         self.fitness(v, &Default::default())
//! #     }
//! #     fn ub(&self) -> &[f64] { &self.1 }
//! #     fn lb(&self) -> &[f64] { &self.0 }
//! # }
//!
//! let a = Solver::solve(
//!     MyFunc::new(),
//!     RgaSetting::default().task(Task::MinFit(1e-20)),
//!     |_| true // Run without callback
//! );
//! let ans: f64 = a.result(); // Get the result from objective function
//! let (x, y): (Array1<f64>, f64) = a.parameters(); // Get the optimized XY value of your function
//! let history: Vec<Report> = a.history(); // Get the history reports
//! ```
//!
//! There are two traits [`Algorithm`](crate::utility::Algorithm) and
//! [`Setting`](crate::utility::Setting).
//! The previous is used to design the optimization method,
//! and the latter is the setting interface.
//!
//! [`Solver`] is a simple interface for obtaining the solution, or analyzing the result.
//!
//! # Objective Function
//!
//! You can define your question as a objective function through implementing [`ObjFunc`].
//!
//! First of all, the array types are [`ndarray::ArrayBase`].
//! And then you should define the upper bound, lower bound, and objective function [`ObjFunc::fitness`] by yourself.
//!
//! The final answer is [`ObjFunc::result`], which is generated from the design parameters.
//!
//! # Random Function
//!
//! This crate use 32bit PRNG algorithm to generate random value, before that,
//! a random seed is required.
//! The seed is generated by `getrandom`, please see its [support platform](getrandom#supported-targets).
//!
//! # Features
//!
//! + `std`: Default feature. Enable standard library function, such as timing and threading.
//! + `parallel`: Enable parallel function, let objective function running without ordered,
//!   uses [`std::thread::spawn`].
//!   Disable it for the platform that doesn't supported threading,
//!   or if your objective function is not complicate enough.
//!   This feature required `std`.
//! + `wasm`: Support for webassembly, especial for random seed generating.
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate core as std;

pub use crate::methods::*;
pub use crate::obj_func::ObjFunc;
pub use crate::report::*;
pub use crate::solver::Solver;
pub use crate::task::Task;
pub use ndarray::{Array1, Array2, AsArray};

/// Define a data structure and its builder functions.
///
/// Use `@` to denote the base settings, such as population number, task category
/// or reporting interval.
/// ```
/// use metaheuristics_nature::{setting_builder, utility::*};
/// # use metaheuristics_nature::ObjFunc;
/// # pub struct GA;
/// # impl Algorithm for GA {
/// #     type Setting = GASetting;
/// #     fn create(settings: &Self::Setting) -> Self { unimplemented!() }
/// #     fn generation<F: ObjFunc>(&mut self, ctx: &mut Context<F>) { unimplemented!() }
/// # }
///
/// setting_builder! {
///     /// Genetic Algorithm settings.
///     pub struct GASetting for GA {
///         @base,
///         @pop_num = 500,
///         cross: f64 = 0.95,
///         mutate: f64 = 0.05,
///         win: f64 = 0.95,
///         delta: f64 = 5.,
///     }
/// }
/// let s = GASetting::default().pop_num(300).cross(0.9);
/// ```
///
/// This macro will also implement [`Setting`](crate::utility::Setting) trait.
#[macro_export]
macro_rules! setting_builder {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $(for $alg:ident)? {
            $(@$base:ident, $(@$base_field:ident = $base_default:literal,)*)?
            $($(#[$field_attr:meta])* $field:ident: $field_ty:ty = $field_default:expr,)*
        }
    ) => {
        $(#[$attr])*
        $vis struct $name {
            $($base: $crate::utility::BasicSetting,)?
            $($field: $field_ty,)*
        }
        impl $name {
            $(setting_builder! {
                @$base,
                /// Termination condition.
                task: $crate::Task,
                /// Population number.
                pop_num: usize,
                /// The report frequency. (per generation)
                rpt: u32,
            })?
            $($(#[$field_attr])* pub fn $field(mut self, $field: $field_ty) -> Self {
                self.$field = $field;
                self
            })*
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($base: $crate::utility::BasicSetting::default()$(.$base_field($base_default))*,)?
                    $($field: $field_default,)*
                }
            }
        }
        $(impl $crate::utility::Setting for $name {
            type Algorithm = $alg;
            fn into_setting(self) -> $crate::utility::BasicSetting {
                self.$base
            }
        })?
    };
    (@$base:ident, $($(#[$field_attr:meta])* $field:ident: $field_type:ty,)+) => {
        $($(#[$field_attr])* pub fn $field(mut self, $field: $field_type) -> Self {
            self.$base = self.$base.$field($field);
            self
        })+
    }
}

mod methods;
mod obj_func;
pub mod random;
mod report;
mod solver;
mod task;
#[cfg(test)]
mod tests;
pub mod thread_pool;
pub mod utility;
