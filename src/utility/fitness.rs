/// The return value of the objective function.
///
/// Usually, the fitness can use [`f64`] / [`f32`] type as the return value.
/// More advanced, any cloneable type that has comparison function can be used.
pub trait Fitness: Sync + Send + Clone + PartialOrd + PartialEq + 'static {
    /// Infinity value of the initial state.
    const INFINITY: Self;
}

impl Fitness for f64 {
    const INFINITY: Self = Self::INFINITY;
}

impl Fitness for f32 {
    const INFINITY: Self = Self::INFINITY;
}
