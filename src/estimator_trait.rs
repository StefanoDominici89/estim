/// Trait for statistical estimators.
pub trait Estimator {
    /// Numeric type associated to the value of an estimator.
    /// It is bound to be a floating point number.
    ///
    type Number: Copy;

    /// Evaluates the estimator and returns the estimated value.
    ///
    fn evaluate(&self) -> Self::Number;

    /// Updates the state of an estimator with an additional sample.
    ///
    fn update(&mut self, sample: Self::Number);

    /// Resets the estimator to a pristine state.
    ///
    fn reset(&mut self);
}
