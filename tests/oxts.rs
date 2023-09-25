use kitti_dataset::oxts;

#[test]
fn parse_oxts() {
    let oxts = oxts::vec_from_path("tests/oxts.txt").unwrap();
    let _text = oxts::write_to_string(oxts).unwrap();
}
