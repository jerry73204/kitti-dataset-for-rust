use crate::{Error, ProjectionMatrix};
use itertools::Itertools;
use std::{
    borrow::Borrow,
    fs::File,
    io::{self, prelude::*, BufReader, BufWriter, Cursor},
    path::Path,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Pose(pub ProjectionMatrix);

impl Pose {
    pub fn iter_from_reader<R>(reader: R) -> impl Iterator<Item = Result<Pose, Error>>
    where
        R: Read,
    {
        let reader = BufReader::new(reader);

        reader.lines().map(|line| -> Result<_, Error> {
            let line = line?;
            let line = line.trim();
            let tokens = line.split_ascii_whitespace();
            let values: Vec<f32> = tokens.map(|tk| tk.parse()).try_collect().unwrap();
            let values: [f32; 12] = values.try_into().unwrap();
            let [r11, r12, r13, tx, r21, r22, r23, ty, r31, r32, r33, tz] = values;
            let mat = [
                [r11, r12, r13, tx],
                [r21, r22, r23, ty],
                [r31, r32, r33, tz],
            ];

            Ok(Pose(ProjectionMatrix(mat)))
        })
    }

    pub fn iter_from_path<P>(path: P) -> Result<impl Iterator<Item = Result<Pose, Error>>, Error>
    where
        P: AsRef<Path>,
    {
        Ok(Self::iter_from_reader(File::open(path)?))
    }

    pub fn iter_from_str(text: &str) -> impl Iterator<Item = Result<Pose, Error>> + '_ {
        let reader = Cursor::new(text);
        Self::iter_from_reader(reader)
    }

    pub fn vec_from_reader<R>(reader: R) -> Result<Vec<Pose>, Error>
    where
        R: Read,
    {
        let poses: Vec<_> = Self::iter_from_reader(reader).try_collect()?;
        Ok(poses)
    }

    pub fn vec_from_path<P>(path: P) -> Result<Vec<Pose>, Error>
    where
        P: AsRef<Path>,
    {
        let poses: Vec<_> = Self::iter_from_path(path)?.try_collect()?;
        Ok(poses)
    }

    pub fn vec_from_str<P>(text: &str) -> Result<Vec<Pose>, Error> {
        let poses: Vec<_> = Self::iter_from_str(text).try_collect()?;
        Ok(poses)
    }

    pub fn write_to_writer<W, I, A>(mut writer: W, poses: I) -> io::Result<()>
    where
        I: IntoIterator<Item = A>,
        W: Write,
        A: Borrow<Pose>,
    {
        for pose in poses {
            let Pose(ProjectionMatrix(
                [[r11, r12, r13, tx], [r21, r22, r23, ty], [r31, r32, r33, tz]],
            )) = *pose.borrow();

            writeln!(
                writer,
                "{r11} {r12} {r13} {tx} {r21} {r22} {r23} {ty} {r31} {r32} {r33} {tz}"
            )?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn write_to_path<P, I, A>(path: P, poses: I) -> io::Result<()>
    where
        I: IntoIterator<Item = A>,
        P: AsRef<Path>,
        A: Borrow<Pose>,
    {
        let writer = BufWriter::new(File::create(path)?);
        Self::write_to_writer(writer, poses)
    }

    pub fn write_to_string<I, A>(poses: I) -> io::Result<String>
    where
        I: IntoIterator<Item = A>,
        A: Borrow<Pose>,
    {
        let mut buf = vec![];
        Self::write_to_writer(&mut buf, poses)?;
        Ok(String::from_utf8(buf).unwrap())
    }
}
