use measurements::{Angle, Length};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializedAnnotation {
    pub class: String,
    pub truncation: f64,
    pub occlusion: Occlusion,
    pub alpha: f64,
    pub xmin: f64,
    pub ymin: f64,
    pub xmax: f64,
    pub ymax: f64,
    pub height: Length,
    pub width: Length,
    pub length: Length,
    pub x: Length,
    pub y: Length,
    pub z: Length,
    pub rotation_y: Angle,
}

impl From<SerializedAnnotation> for Annotation {
    fn from(from: SerializedAnnotation) -> Self {
        let SerializedAnnotation {
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
            alpha,
            bbox: BoundingBox {
                xmin,
                ymin,
                xmax,
                ymax,
            },
            extents: Extents {
                height,
                width,
                length,
            },
            location: Location { x, y, z },
            rotation_y,
        }
    }
}

impl From<Annotation> for SerializedAnnotation {
    fn from(from: Annotation) -> Self {
        let Annotation {
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

        SerializedAnnotation {
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "SerializedAnnotation", into = "SerializedAnnotation")]
pub struct Annotation {
    pub class: String,
    pub truncation: f64,
    pub occlusion: Occlusion,
    pub alpha: f64,
    pub bbox: BoundingBox,
    pub extents: Extents,
    pub location: Location,
    pub rotation_y: Angle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
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
