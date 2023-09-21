use kitti_dataset::{CameraCalibration, OdometryCalibration};

#[test]
fn parse_calib() {
    let _ = CameraCalibration::from_path("tests/camera_calib.txt").unwrap();
    let _ = OdometryCalibration::from_path("tests/odometry_calib.txt").unwrap();
}
