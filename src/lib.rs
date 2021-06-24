//! Collection of lightweight statistical estimators accessible through the shared Estimator trait.
//! 
mod estimator_lib;
mod estimator_trait;

pub mod traits {
    pub use crate::estimator_trait::Estimator;
}

pub mod estimators {
    pub use crate::estimator_lib::Average;
    pub use crate::estimator_lib::MeanSquaredError;
}
