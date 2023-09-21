use kitti_dataset::CameraCalibration;

#[test]
fn parse_calib() {
    let _calib = CameraCalibration::from_path("tests/calib.txt").unwrap();
}
