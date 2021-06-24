use crate::estimator_trait::Estimator;

/// Estimates the average of a serie of elements.
pub struct Average<T> {
    /// The internal state of the estimator.
    state: T,
    /// The total number of accounted samples.
    num_samples: usize,
}

macro_rules! impl_for_average {
    ($float:ty) => {
        impl Average<$float> {
            pub fn new() -> Self {
                Average {
                    state: 0.0,
                    num_samples: 0,
                }
            }
        }
    };
}

macro_rules! impl_estimator_for_average {
    ($float:ty) => {
        impl Estimator for Average<$float> {
            type Number = $float;

            fn evaluate(&self) -> Self::Number {
                self.state
            }

            fn update(&mut self, sample: Self::Number) {
                let old_num_samples = self.num_samples as Self::Number;
                self.num_samples += 1;
                let new_num_samples = self.num_samples as Self::Number;
                let ratio_old_to_new = old_num_samples / new_num_samples;
                self.state *= ratio_old_to_new;
                self.state += sample / new_num_samples;
            }

            fn reset(&mut self) {
                *self = Average::<Self::Number>::new();
            }
        }
    };
}

impl_for_average!(f32);
impl_for_average!(f64);

impl_estimator_for_average!(f32);
impl_estimator_for_average!(f64);

/// Estimates the mean squared error associated to the average of a serie of elements.
pub struct MeanSquaredError<T> {
    /// The associated average.
    pub average: Average<T>,
    /// The internal state.
    state: T,
}

macro_rules! impl_for_squared_mean_squared_error {
    ($float:ty) => {
        impl MeanSquaredError<$float> {
            pub fn new() -> Self {
                MeanSquaredError {
                    average: Average::<$float>::new(),
                    state: 0.0,
                }
            }
        }
    };
}

macro_rules! impl_estimator_for_squared_mean_squared_error {
    ($float:ty) => {
        impl Estimator for MeanSquaredError<$float> {
            type Number = $float;

            fn evaluate(&self) -> Self::Number {
                (self.state - self.average.state * self.average.state)
            }

            fn update(&mut self, sample: Self::Number) {
                let old_num_samples = self.average.num_samples as Self::Number;
                self.average.num_samples += 1;
                let new_num_samples = self.average.num_samples as Self::Number;
                let ratio_old_to_new = old_num_samples / new_num_samples;
                self.state *= ratio_old_to_new;
                self.state += sample * sample / new_num_samples;
                self.average.state *= ratio_old_to_new;
                self.average.state += sample / new_num_samples;
            }

            fn reset(&mut self) {
                *self = MeanSquaredError::<Self::Number>::new();
            }
        }
    };
}

impl_for_squared_mean_squared_error!(f32);
impl_for_squared_mean_squared_error!(f64);

impl_estimator_for_squared_mean_squared_error!(f32);
impl_estimator_for_squared_mean_squared_error!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    // average tests
    macro_rules! impl_test_initial_average_state {
        ($float:ty) => {
            let average = Average::<$float>::new();
            assert_eq!(average.state, 0.0);
            assert_eq!(average.num_samples, 0);
        };
    }

    macro_rules! impl_test_unit_average {
        ($float:ty) => {
            let mut average = Average::<$float>::new();
            for _ in 0..10 {
                average.update(1.0)
            }
            assert_eq!(average.state, 1.0);
            assert_eq!(average.num_samples, 10);
            assert_eq!(average.evaluate(), 1.0);
        };
    }

    macro_rules! impl_test_average {
        ($float:ty) => {
            let mut average = Average::<$float>::new();
            for i in 0..5 {
                average.update(i as $float)
            }
            assert_eq!(average.state, 2.0);
            assert_eq!(average.num_samples, 5);
            assert_eq!(average.evaluate(), 2.0);
        };
    }

    #[test]
    fn initial_average_state_f32() {
        impl_test_initial_average_state!(f32);
    }
    #[test]
    fn initial_average_state_f64() {
        impl_test_initial_average_state!(f64);
    }

    #[test]
    fn unit_average_f32() {
        impl_test_unit_average!(f32);
    }
    #[test]
    fn unit_average_f64() {
        impl_test_unit_average!(f64);
    }

    #[test]
    fn average_f32() {
        impl_test_average!(f32);
    }
    #[test]
    fn average_f64() {
        impl_test_average!(f64);
    }

    // standard deviation tests
    macro_rules! impl_test_initial_squared_mean_squared_error_state {
        ($float:ty) => {
            let stddev = MeanSquaredError::<$float>::new();
            assert_eq!(stddev.state, 0.0);
            assert_eq!(stddev.average.state, 0.0);
            assert_eq!(stddev.average.num_samples, 0);
        };
    }

    macro_rules! impl_test_zero_squared_mean_squared_error {
        ($float:ty) => {
            let mut stddev = MeanSquaredError::<$float>::new();
            for _ in 0..10 {
                stddev.update(1.0)
            }
            assert_eq!(stddev.average.state, 1.0);
            assert_eq!(stddev.average.num_samples, 10);
            assert_eq!(stddev.evaluate(), 0.0);
            assert_eq!(stddev.state, 1.0);
            assert_eq!(stddev.average.evaluate(), 1.0);
        };
    }

    macro_rules! impl_test_squared_mean_squared_error {
        ($float:ty) => {
            let mut stddev = MeanSquaredError::<$float>::new();
            for i in 0..5 {
                stddev.update(i as $float)
            }
            assert_eq!(stddev.average.state, 2.0);
            assert_eq!(stddev.average.num_samples, 5);
            assert_eq!(stddev.evaluate(), (6.0 - 4.0));
        };
    }

    #[test]
    fn initial_squared_mean_squared_error_state_f32() {
        impl_test_initial_squared_mean_squared_error_state!(f32);
    }
    #[test]
    fn initial_squared_mean_squared_error_state_f64() {
        impl_test_initial_squared_mean_squared_error_state!(f64);
    }

    #[test]
    fn zero_squared_mean_squared_error_f32() {
        impl_test_zero_squared_mean_squared_error!(f32);
    }

    #[test]
    fn zero_squared_mean_squared_error_f64() {
        impl_test_zero_squared_mean_squared_error!(f64);
    }

    #[test]
    fn mean_squared_error_f32() {
        impl_test_squared_mean_squared_error!(f32);
    }

    #[test]
    fn mean_squared_error_f64() {
        impl_test_squared_mean_squared_error!(f64);
    }
}
