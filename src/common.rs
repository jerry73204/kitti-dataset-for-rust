#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionMatrix(pub [[f32; 4]; 3]);

#[derive(Debug, Clone, PartialEq)]
pub struct Transform2D(pub [[f32; 3]; 3]);

#[cfg(feature = "nalgebra")]
mod with_nalgebra {
    use super::{ProjectionMatrix, Transform2D};
    use nalgebra::{Matrix3, Matrix3x4};
    use slice_of_array::prelude::*;

    impl ProjectionMatrix {
        pub fn to_na_matrix(&self) -> Matrix3x4<f32> {
            Matrix3x4::from_row_slice(self.0.flat())
        }
    }

    impl Transform2D {
        pub fn to_na_matrix(&self) -> Matrix3<f32> {
            Matrix3::from_row_slice(self.0.flat())
        }
    }
}
