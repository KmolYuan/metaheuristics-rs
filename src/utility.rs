//! The utility API for advanced usage.
//!
//! There is a [`prelude`] module that will import all the items of this crate.
//!
//! ```
//! use metaheuristics_nature::utility::prelude::*;
//! ```
pub use self::{ctx::*, solver_builder::*};

mod ctx;
mod solver_builder;

/// A prelude module for algorithm implementation.
///
/// This module includes all items of this crate, some hidden types,
/// and external items from "ndarray" and "rayon" (if `rayon` feature enabled).
pub mod prelude {
    pub use super::*;
    pub use crate::{random::*, *};

    #[doc(no_inline)]
    pub use crate::ndarray::prelude::*;
    #[cfg(feature = "rayon")]
    #[doc(no_inline)]
    pub use crate::rayon::prelude::*;
    #[cfg(not(feature = "std"))]
    pub use num_traits::Float as _;
}
