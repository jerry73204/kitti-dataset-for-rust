use kitti_dataset::tracking::Label;

#[test]
fn parse_tracking_label() {
    let _: Vec<Label> = Label::vec_from_path("tests/tracking_label.txt").unwrap();
}
