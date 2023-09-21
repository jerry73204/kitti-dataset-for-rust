use slice_of_array::prelude::*;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct CameraCalibration {
    pub p0: ProjectionMatrix,
    pub p1: ProjectionMatrix,
    pub p2: ProjectionMatrix,
    pub p3: ProjectionMatrix,
    pub r0_rect: Transform2D,
    pub tr_velo_to_cam: ProjectionMatrix,
    pub tr_imu_to_velo: ProjectionMatrix,
}

impl Display for CameraCalibration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self {
            p0,
            p1,
            p2,
            p3,
            r0_rect,
            tr_velo_to_cam,
            tr_imu_to_velo,
        } = self;

        write_line(f, "P0", &p0.0)?;
        write_line(f, "P1", &p1.0)?;
        write_line(f, "P2", &p2.0)?;
        write_line(f, "P3", &p3.0)?;
        write_line(f, "R0_rect", &r0_rect.0)?;
        write_line(f, "Tr_velo_to_cam", &tr_velo_to_cam.0)?;
        write_line(f, "Tr_imu_to_velo", &tr_imu_to_velo.0)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OdometryCalibration {
    pub p0: ProjectionMatrix,
    pub p1: ProjectionMatrix,
    pub p2: ProjectionMatrix,
    pub p3: ProjectionMatrix,
    pub tr: ProjectionMatrix,
}

impl Display for OdometryCalibration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self { p0, p1, p2, p3, tr } = self;

        write_line(f, "P0", &p0.0)?;
        write_line(f, "P1", &p1.0)?;
        write_line(f, "P2", &p2.0)?;
        write_line(f, "P3", &p3.0)?;
        write_line(f, "Tr", &tr.0)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionMatrix(pub [[f32; 4]; 3]);

#[derive(Debug, Clone, PartialEq)]
pub struct Transform2D(pub [[f32; 3]; 3]);

fn write_line<const M: usize, const N: usize>(
    f: &mut Formatter<'_>,
    name: &str,
    array: &[[f32; N]; M],
) -> Result<(), fmt::Error> {
    write!(f, "{name}:")?;

    let slice = array.flat();
    for &val in slice {
        write!(f, " {val:.12e}")?;
    }

    writeln!(f)?;
    Ok(())
}

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
