use kitti_dataset::{object, odometry, tracking};

#[test]
fn parse_calib() {
    let _ = object::Calibration::from_path("tests/object_calib.txt").unwrap();
    let _ = tracking::Calibration::from_path("tests/tracking_calib.txt").unwrap();
    let _ = odometry::Calibration::from_path("tests/odometry_calib.txt").unwrap();
}
