use kitti_dataset::object::{BoundingBox, Extents, Label, Location, Occlusion};
use measurements::{Angle, Length};

#[test]
fn parse_object_label() {
    let label1: Vec<Label> = Label::vec_from_path("tests/object_label.txt").unwrap();
    let expect = vec![
        Label {
            class: "Car".to_string(),
            truncation: Some(0.0.try_into().unwrap()),
            occlusion: Some(Occlusion::FullyVisible),
            alpha: Angle::from_radians(-1.58),
            bbox: BoundingBox {
                xmin: 587.01,
                ymin: 173.33,
                xmax: 614.12,
                ymax: 200.12,
            },
            extents: Extents {
                height: Length::from_meters(1.65),
                width: Length::from_meters(1.67),
                length: Length::from_meters(3.64),
            },
            location: Location {
                x: Length::from_meters(-0.65),
                y: Length::from_meters(1.71),
                z: Length::from_meters(46.70),
            },
            rotation_y: Angle::from_radians(-1.59),
        },
        Label {
            class: "Cyclist".to_string(),
            truncation: Some(0.0.try_into().unwrap()),
            occlusion: Some(Occlusion::FullyVisible),
            alpha: Angle::from_radians(-2.46),
            bbox: BoundingBox {
                xmin: 665.45,
                ymin: 160.00,
                xmax: 717.93,
                ymax: 217.99,
            },
            extents: Extents {
                height: Length::from_meters(1.72),
                width: Length::from_meters(0.47),
                length: Length::from_meters(1.65),
            },
            location: Location {
                x: Length::from_meters(2.45),
                y: Length::from_meters(1.35),
                z: Length::from_meters(22.10),
            },
            rotation_y: Angle::from_radians(-2.35),
        },
        Label {
            class: "Pedestrian".to_string(),
            truncation: Some(0.0.try_into().unwrap()),
            occlusion: Some(Occlusion::LargelyOccluded),
            alpha: Angle::from_radians(0.21),
            bbox: BoundingBox {
                xmin: 423.17,
                ymin: 173.67,
                xmax: 433.17,
                ymax: 224.03,
            },
            extents: Extents {
                height: Length::from_meters(1.60),
                width: Length::from_meters(0.38),
                length: Length::from_meters(0.30),
            },
            location: Location {
                x: Length::from_meters(-5.87),
                y: Length::from_meters(1.63),
                z: Length::from_meters(23.11),
            },
            rotation_y: Angle::from_radians(-0.03),
        },
    ];
    assert_eq!(label1, expect);

    let text = Label::write_to_string(&label1).unwrap();
    let label2 = Label::vec_from_str(&text).unwrap();
    assert_eq!(label1, label2);
}
