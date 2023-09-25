use kitti_dataset::tracking::{BoundingBox, Class, Extents, Label, Location, Occlusion};

#[test]
fn parse_tracking_label() {
    let label1: Vec<Label> = Label::vec_from_path("tests/tracking_label.txt").unwrap();
}
