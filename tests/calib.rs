use kitti_dataset::{
    object::ObjectCalibration, odometry::OdometryCalibration, tracking::TrackingCalibration,
};

#[test]
fn parse_calib() {
    let _ = ObjectCalibration::from_path("tests/object_calib.txt").unwrap();
    let _ = TrackingCalibration::from_path("tests/tracking_calib.txt").unwrap();
    let _ = OdometryCalibration::from_path("tests/odometry_calib.txt").unwrap();
}
