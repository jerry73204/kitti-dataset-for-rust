use kitti_dataset::tracking::Oxts;

#[test]
fn parse_oxts() {
    let oxts = Oxts::vec_from_path("tests/oxts.txt").unwrap();
    let _text = Oxts::write_to_string(oxts).unwrap();
}
