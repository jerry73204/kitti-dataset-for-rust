use kitti_dataset::common::{Point, ProjectionMatrix, Transform2D};
use nalgebra::{Matrix3, Matrix3x4, Point3, Transform2};
use slice_of_array::prelude::*;

pub trait PointNalgebraExt {
    fn to_na_xyz_point(&self) -> Point3<f32>;
}

impl PointNalgebraExt for Point {
    fn to_na_xyz_point(&self) -> Point3<f32> {
        let Self { x, y, z, .. } = *self;
        Point3::new(x, y, z)
    }
}

pub trait ProjectionMatrixNalgebraExt {
    fn to_na_matrix(&self) -> Matrix3x4<f32>;
}

impl ProjectionMatrixNalgebraExt for ProjectionMatrix {
    /// Convert to nalgebra [Matrix3x4].
    fn to_na_matrix(&self) -> Matrix3x4<f32> {
        Matrix3x4::from_row_slice(self.0.flat())
    }
}

pub trait Transform2DNalgebraExt {
    fn to_na_matrix(&self) -> Matrix3<f32>;
    fn to_na_transform(&self) -> Transform2<f32>;
}

impl Transform2DNalgebraExt for Transform2D {
    /// Convert to nalgebra [Matrix3].
    fn to_na_matrix(&self) -> Matrix3<f32> {
        Matrix3::from_row_slice(self.0.flat())
    }

    fn to_na_transform(&self) -> Transform2<f32> {
        Transform2::from_matrix_unchecked(self.to_na_matrix())
    }
}
