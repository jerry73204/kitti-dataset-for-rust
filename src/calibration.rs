use chrono::NaiveDateTime;
#[cfg(feature = "nalgebra")]
use nalgebra as na;

// #[derive(Debug, Clone, PartialEq)]
// pub struct CamToCam {
//     pub time: NaiveDateTime,
//     pub corner_dist: f64,
//     pub rotation: [f64; 9],
//     pub translation: [f64; 3],
// }

#[derive(Debug, Clone, PartialEq)]
pub struct ImuToVelo {
    pub time: NaiveDateTime,
    pub rotation: [f64; 9],
    pub translation: [f64; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct VeloToCam {
    pub time: NaiveDateTime,
    pub rotation: [f64; 9],
    pub translation: [f64; 3],
    pub delta_f: [f64; 2],
    pub delta_c: [f64; 2],
}

#[cfg(feature = "nalgebra")]
pub trait NalgebraCalib {
    fn rotation_array(&self) -> [f64; 9];
    fn translation_array(&self) -> [f64; 3];

    fn na_isometry(&self) -> na::IsometryMatrix3<f64> {
        na::IsometryMatrix3 {
            translation: self.na_translation(),
            rotation: self.na_rotation_matrix(),
        }
    }

    fn na_rotation_matrix(&self) -> na::Rotation3<f64> {
        let mat = na::Matrix3::from_row_iterator(self.rotation_array());
        na::Rotation3::from_matrix(&mat)
    }

    fn na_translation(&self) -> na::Translation3<f64> {
        let vec = na::Vector3::from(self.translation_array());
        na::Translation3::from(vec)
    }
}

#[cfg(feature = "nalgebra")]
impl NalgebraCalib for VeloToCam {
    fn rotation_array(&self) -> [f64; 9] {
        self.rotation
    }

    fn translation_array(&self) -> [f64; 3] {
        self.translation
    }
}

#[cfg(feature = "nalgebra")]
impl NalgebraCalib for ImuToVelo {
    fn rotation_array(&self) -> [f64; 9] {
        self.rotation
    }

    fn translation_array(&self) -> [f64; 3] {
        self.translation
    }
}
