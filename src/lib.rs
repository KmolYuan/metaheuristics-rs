//! A collection of nature-inspired meta-heuristic algorithms.
//!
//! # Algorithm
//!
//! There are two traits [`Algorithm`](crate::utility::Algorithm) and [`Setting`].
//! The previous is used to design the optimization method,
//! and the latter is the setting interface.
//!
//! [`Solver`] is a simple interface for obtaining the solution, or analyzing the result.
//! This type allows you to use the API without importing any traits.
//!
//! All provided methods are listed in the module [`methods`].
//!
//! For making your owned method, please see [`utility::prelude`].
//!
//! # Objective Function
//!
//! You can define your question as a objective function through implementing [`ObjFunc`],
//! and then the upper bound, lower bound, and objective function [`ObjFunc::fitness`] should be defined.
//!
//! The final answer is [`ObjFunc::result`], which is calculated from the design parameters.
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
//! + `parallel`: Enable parallel function, let objective function running without ordered, uses [`rayon`].
//!   Disable it for the platform that doesn't supported threading,
//!   or if your objective function is not complicate enough.
//!   This feature required `std`.
//! + `wasm`: Support for webassembly, especial for random seed generating.
//! + `libm`: If the standard library is not provided, some math functions might missing.
//!   This will disable some pre-implemented algorithms.
//!   However, there is a math library implemented in pure Rust, the name is same as `libm`.
//!   This feature can re-enable (or replace) the math functions by using the `libm` crate.
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate core as std;

pub use crate::{
    methods::*,
    obj_func::ObjFunc,
    report::Report,
    solver::Solver,
    utility::setting::{Adaptive, Setting, Task},
};

/// A tool macro used to build the builder function.
///
/// The macro will generate following code:
///
/// ```
/// # type Ty = bool;
/// # struct S {
/// #     name: Ty,
/// # }
/// # impl S {
/// #[doc = "description"]
/// pub fn name(mut self, name: Ty) -> Self {
///     self.name = name;
///     self
/// }
/// # }
/// ```
#[macro_export]
macro_rules! impl_builder {
    ($name:ident, $ty:ty, $description:literal) => {
        #[doc = $description]
        pub fn $name(mut self, $name: $ty) -> Self {
            self.$name = $name;
            self
        }
    };
}

pub mod methods;
mod obj_func;
pub mod random;
mod report;
mod solver;
#[cfg(test)]
mod tests;
pub mod thread_pool;
pub mod utility;
