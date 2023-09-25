use crate::serde::{occlusion, truncation};
use measurements::{Angle, Length};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "SerializedLabel", into = "SerializedLabel")]
pub struct Label {
    pub class: Class,
    pub truncation: Option<f64>,
    pub occlusion: Option<Occlusion>,
    pub alpha: Angle,
    pub bbox: BoundingBox,
    pub extents: Extents,
    pub location: Location,
    pub rotation_y: Angle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializedLabel {
    pub class: Class,
    #[serde(with = "truncation")]
    pub truncation: Option<f64>,
    #[serde(with = "occlusion")]
    pub occlusion: Option<Occlusion>,
    pub alpha: f64,
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
    pub height: f64,
    pub width: f64,
    pub length: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation_y: f64,
}

impl From<SerializedLabel> for Label {
    fn from(from: SerializedLabel) -> Self {
        let SerializedLabel {
            class,
            truncation,
            occlusion,
            alpha,
            xmin,
            ymin,
            xmax,
            ymax,
            height,
            width,
            length,
            x,
            y,
            z,
            rotation_y,
        } = from;

        Self {
            class,
            truncation,
            occlusion,
            alpha: Angle::from_radians(alpha),
            bbox: BoundingBox {
                xmin,
                ymin,
                xmax,
                ymax,
            },
            extents: Extents {
                height: Length::from_meters(height),
                width: Length::from_meters(width),
                length: Length::from_meters(length),
            },
            location: Location {
                x: Length::from_meters(x),
                y: Length::from_meters(y),
                z: Length::from_meters(z),
            },
            rotation_y: Angle::from_radians(rotation_y),
        }
    }
}

impl From<Label> for SerializedLabel {
    fn from(from: Label) -> Self {
        let Label {
            class,
            truncation,
            occlusion,
            alpha,
            bbox:
                BoundingBox {
                    xmin,
                    ymin,
                    xmax,
                    ymax,
                },
            extents:
                Extents {
                    height,
                    width,
                    length,
                },
            location: Location { x, y, z },
            rotation_y,
        } = from;

        SerializedLabel {
            class,
            truncation,
            occlusion,
            alpha: alpha.as_radians(),
            xmin,
            ymin,
            xmax,
            ymax,
            height: height.as_meters(),
            width: width.as_meters(),
            length: length.as_meters(),
            x: x.as_meters(),
            y: y.as_meters(),
            z: z.as_meters(),
            rotation_y: rotation_y.as_radians(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum Occlusion {
    FullyVisible = 0,
    PartlyVisible = 1,
    LargelyOccluded = 2,
    Unknown = 3,
}

/// Describes the bounding box range in the image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Extents {
    pub height: Length,
    pub width: Length,
    pub length: Length,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
)]
pub enum Class {
    Car,
    Van,
    Truck,
    Pedestrian,
    #[serde(rename = "Person_sitting")]
    #[strum(serialize = "Person_sitting")]
    PersonSitting,
    Cyclist,
    Tram,
    Misc,
    DontCare,
}
