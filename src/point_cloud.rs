use crate::Error;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Cursor},
    path::Path,
};

pub fn vec_from_reader<R: Read>(mut reader: R) -> Result<Vec<Point>, Error> {
    let try_read_f32 = |reader: &mut R| -> Result<Option<f32>, Error> {
        let mut bytes = [0u8; 4];
        let mut buf = &mut bytes[..];

        let recv = reader.read(buf)?;
        if recv == 0 {
            return Ok(None);
        }
        buf = &mut buf[recv..];

        reader.read_exact(buf)?;

        let value = f32::from_le_bytes(bytes);
        Ok(Some(value))
    };

    let read_f32 = |reader: &mut R| -> Result<f32, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let value = f32::from_le_bytes(buf);
        Ok(value)
    };

    let mut points = vec![];

    loop {
        let Some(x) = try_read_f32(&mut reader)? else {
            break;
        };
        let y = read_f32(&mut reader)?;
        let z = read_f32(&mut reader)?;
        let reflection = read_f32(&mut reader)?;

        let point = Point {
            x,
            y,
            z,
            reflection,
        };
        points.push(point)
    }

    Ok(points)
}

pub fn vec_from_path<P>(path: P) -> Result<Vec<Point>, Error>
where
    P: AsRef<Path>,
{
    let reader = BufReader::new(File::open(path)?);
    vec_from_reader(reader)
}

pub fn vec_from_bytes(bytes: &[u8]) -> Result<Vec<Point>, Error>
where
{
    let reader = Cursor::new(bytes);
    vec_from_reader(reader)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub reflection: f32,
}

impl Point {
    pub fn xyz(&self) -> [f32; 3] {
        let Self { x, y, z, .. } = *self;
        [x, y, z]
    }

    pub fn xyzr(&self) -> [f32; 4] {
        let Self {
            x,
            y,
            z,
            reflection,
        } = *self;
        [x, y, z, reflection]
    }
}

#[cfg(feature = "nalgebra")]
mod with_nalgebra {
    use nalgebra::Point3;

    impl super::Point {
        pub fn to_na_xyz_point(&self) -> Point3<f32> {
            let Self { x, y, z, .. } = *self;
            Point3::new(x, y, z)
        }
    }
}
