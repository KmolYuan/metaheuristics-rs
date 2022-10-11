//! A collection of nature-inspired meta-heuristic algorithms.
//!
//! # Terminology
//!
//! For unifying the terms, in this documentation,
//!
//! + "Iteration" is called "generation".
//! + "Function" that evaluates value is called "objective function".
//! + "Return value" of the objective function is called "fitness".
//!
//! # Algorithm
//!
//! There are two traits [`Algorithm`] and [`Setting`].
//! The previous is used to design the optimization method,
//! and the latter is the setting interface.
//!
//! [`Solver`] is a simple interface for obtaining the solution, or analyzing
//! the result. This type allows you to use the pre-defined methods without
//! importing any traits.
//!
//! All provided methods are listed in the module [`methods`].
//!
//! For making your owned method, please see [`utility::prelude`].
//!
//! # Objective Function
//!
//! For a quick demo with callable object, please see [`Fx`].
//!
//! You can define your question as an objective function through implementing
//! [`ObjFunc`], and then the upper bound, lower bound, and objective function
//! [`ObjFunc::fitness()`] should be defined.
//!
//! A high level trait is [`ObjFactory`], its final answer is
//! [`ObjFactory::Product`], which is calculated from the design parameters.
//!
//! # Random Function
//!
//! This crate use 64bit PRNG algorithm ([`utility::Rng`]) to generate uniform
//! random values, before that, a random seed is required.
//! The seed is generated by `getrandom`, please see its [support
//! platform](getrandom#supported-targets).
//!
//! # Features
//!
//! + `std`: Default feature. Enable standard library function, such as timing
//! and threading.   If `std` is disabled, crate "libm" will be enabled for the
//! math functions. + `rayon`: Enable parallel computation via `rayon`, let
//! objective function running without ordered.   Disable it for the platform
//! that doesn't supported threading,   or if your objective function is not
//! complicate enough.   This feature require `std` feature.
//!
//! # Compatibility
//!
//! If you are using this crate for providing objective function,
//! other downstream crates of yours may have some problems with compatibility.
//!
//! The most important thing is using a stable version, specifying the major
//! version number. Then re-export (`pub use`) this crate for the downstream
//! crates.
//!
//! This crate does the same things on `ndarray` and `rayon`.
#![cfg_attr(doc_cfg, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate core as std;

pub use self::{algorithm::*, fx_func::*, methods::*, obj_func::*, setting::*, solver::*};

/// A tool macro used to generate multiple builder functions (methods).
///
/// For example,
///
/// ```
/// # use metaheuristics_nature::impl_builders;
/// # type Ty = bool;
/// # struct S {
/// #     name1: Ty,
/// #     name2: Ty,
/// # }
/// impl S {
///     impl_builders! {
///         /// Doc 1
///         fn name1(Ty)
///         /// Doc 2
///         fn name2(Ty)
///     }
/// }
/// ```
///
/// will become
///
/// ```
/// # type Ty = bool;
/// # struct S {
/// #     name1: Ty,
/// #     name2: Ty,
/// # }
/// impl S {
///     /// Doc 1
///     pub fn name1(mut self, name1: Ty) -> Self {
///         self.name1 = name1;
///         self
///     }
///     /// Doc 2
///     pub fn name2(mut self, name2: Ty) -> Self {
///         self.name2 = name2;
///         self
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_builders {
    ($($(#[$meta:meta])* fn $name:ident($ty:ty))+) => {$(
        $(#[$meta])*
        pub fn $name(self, $name: $ty) -> Self {
            Self { $name, ..self }
        }
    )+};
}

mod algorithm;
mod fx_func;
pub mod methods;
mod obj_func;
mod setting;
mod solver;
pub mod tests;
pub mod utility;
pub mod ndarray {
    //! The re-export of the crate `ndarray`.
    #[doc(no_inline)]
    pub use ndarray::*;
}
pub mod rayon {
    //! The re-export of the crate `rayon`.
    #![cfg(feature = "rayon")]
    #[doc(no_inline)]
    pub use rayon::*;
}
