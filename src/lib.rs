pub mod annotation;
pub mod calibration;
pub mod error;

pub use calibration::*;
pub use annotation::{Annotation, BoundingBox, Extents, Location, Occlusion};
pub use error::Error;

pub mod prelude {
    #[cfg(feature = "nalgebra")]
    pub use crate::calibration::NalgebraCalib as _;
}
