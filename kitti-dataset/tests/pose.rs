use kitti_dataset::odometry::Pose;

#[test]
fn parse_pose() {
    let _ = Pose::vec_from_path("tests/pose.txt").unwrap();
}
