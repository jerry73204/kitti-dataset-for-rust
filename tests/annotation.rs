use kitti_dataset::{Annotation, BoundingBox, Location, Size};
use measurements::{Angle, Length};

#[test]
fn kitti_annotation_parsing_test() {
    let anns1: Vec<Annotation> = kitti_dataset::vec::from_path("tests/annotation.txt").unwrap();
    let expect = vec![
        Annotation {
            class: "car".into(),
            truncation: 0.0,
            occlusion: kitti_dataset::Occlusion::FullyVisible,
            alpha: -1.58,
            bbox: BoundingBox {
                xmin: 587.01,
                ymin: 173.33,
                xmax: 614.12,
                ymax: 200.12,
            },
            size: Size {
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
        Annotation {
            class: "cyclist".into(),
            truncation: 0.0,
            occlusion: kitti_dataset::Occlusion::FullyVisible,
            alpha: -2.46,
            bbox: BoundingBox {
                xmin: 665.45,
                ymin: 160.00,
                xmax: 717.93,
                ymax: 217.99,
            },
            size: Size {
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
        Annotation {
            class: "pedestrian".into(),
            truncation: 0.0,
            occlusion: kitti_dataset::Occlusion::LargelyOccluded,
            alpha: 0.21,
            bbox: BoundingBox {
                xmin: 423.17,
                ymin: 173.67,
                xmax: 433.17,
                ymax: 224.03,
            },
            size: Size {
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
    assert_eq!(anns1, expect);

    let text = kitti_dataset::write::to_string(&anns1).unwrap();
    let anns2 = kitti_dataset::vec::from_str(&text).unwrap();
    assert_eq!(anns1, anns2);
}
