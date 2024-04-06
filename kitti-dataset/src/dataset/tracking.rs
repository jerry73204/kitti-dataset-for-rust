use crate::{
    common::PointCloud,
    tracking::{Calibration, Label, Oxts},
    Error,
};
use image::DynamicImage;
use itertools::Itertools;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct TrackingDataset {
    dataset_dir: PathBuf,
    num_frames: usize,
    sub_dirs: HashMap<String, DataKind>,
}

impl TrackingDataset {
    pub fn open<P>(dir: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let dataset_dir = dir.as_ref();

        let sub_dirs: HashMap<String, DataKind> = dataset_dir
            .read_dir()?
            .map(|entry| -> Result<_, Error> {
                macro_rules! skip {
                    () => {
                        return Ok(None);
                    };
                }

                let entry = entry?;
                let path = entry.path();

                if !path.canonicalize()?.is_dir() {
                    skip!();
                }

                let Some(file_name) = path.file_name() else {
                    skip!();
                };
                let Some(file_name) = file_name.to_str() else {
                    skip!();
                };

                let kind = if file_name.starts_with("image") {
                    DataKind::ImageSeq
                } else if file_name.starts_with("velodyne") {
                    DataKind::VelodyneSeq
                } else if file_name.starts_with("label") {
                    DataKind::Label
                } else if file_name.starts_with("calib") {
                    DataKind::Calib
                } else if file_name.starts_with("oxts") {
                    DataKind::Odomentry
                } else {
                    skip!();
                };

                Ok(Some((file_name.to_string(), kind)))
            })
            .flatten_ok()
            .try_collect()?;

        let num_frames = match sub_dirs.iter().next() {
            Some((key, kind)) => {
                super::probe_max_frames(&dataset_dir.join(key), 4, kind.file_ext())
            }
            None => 0,
        };

        Ok(Self {
            num_frames,
            sub_dirs,
            dataset_dir: dataset_dir.to_owned(),
        })
    }

    pub fn num_frames(&self) -> usize {
        self.num_frames
    }

    pub fn frame_iter(&self) -> impl Iterator<Item = Frame<'_>> {
        (0..self.num_frames).map(|frame_idx| Frame {
            dataset: self,
            frame_idx,
        })
    }

    pub fn frame(&self, frame_idx: usize) -> Option<Frame<'_>> {
        if frame_idx >= self.num_frames {
            return None;
        }

        Some(Frame {
            dataset: self,
            frame_idx,
        })
    }

    pub fn key(&self, key: &str) -> Option<KeyEntry<'_>> {
        let kind = *self.sub_dirs.get(key)?;
        Some(KeyEntry {
            dataset: self,
            key: key.to_string(),
            kind,
        })
    }

    pub fn keys(&self) -> impl Iterator<Item = (&str, DataKind)> {
        self.sub_dirs.iter().map(|(s, &k)| (s.as_str(), k))
    }
}

#[derive(Debug, Clone)]
pub struct Frame<'a> {
    dataset: &'a TrackingDataset,
    frame_idx: usize,
}

impl<'a> Frame<'a> {
    pub fn key(&self, key: &str) -> Option<Sample> {
        let kind = *self.dataset.sub_dirs.get(key)?;
        let file_name = create_file_name(self.frame_idx, kind.file_ext());
        let path = self.dataset.dataset_dir.join(key).join(file_name);
        let sample = Sample { kind, path };
        Some(sample)
    }

    pub fn sample_iter(&self) -> impl Iterator<Item = Sample> + '_ {
        self.dataset.sub_dirs.iter().map(|(key, &kind)| {
            let file_name = create_file_name(self.frame_idx, kind.file_ext());
            let path = self.dataset.dataset_dir.join(key).join(file_name);
            Sample { kind, path }
        })
    }

    pub fn seq_len(&self) -> Option<usize> {
        let (key, kind) = self
            .dataset
            .sub_dirs
            .iter()
            .find(|(_, kind)| [DataKind::ImageSeq, DataKind::VelodyneSeq].contains(kind))?;

        let seq_dir = self
            .dataset
            .dataset_dir
            .join(key)
            .join(format!("{:04}", self.frame_idx));
        let ext = match kind {
            DataKind::ImageSeq => "png",
            DataKind::VelodyneSeq => "bin",
            _ => unreachable!(),
        };
        let seq_len = super::probe_max_frames(&seq_dir, 6, Some(ext));
        Some(seq_len)
    }
}

#[derive(Debug, Clone)]
pub struct KeyEntry<'a> {
    dataset: &'a TrackingDataset,
    key: String,
    kind: DataKind,
}

impl<'a> KeyEntry<'a> {
    pub fn frame(&self, frame_idx: usize) -> Option<Sample> {
        if frame_idx >= self.dataset.num_frames {
            return None;
        }

        let file_name = create_file_name(frame_idx, self.kind.file_ext());
        let path = self.dataset.dataset_dir.join(&self.key).join(file_name);

        let sample = Sample {
            kind: self.kind,
            path,
        };
        Some(sample)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataKind {
    ImageSeq,
    VelodyneSeq,
    Calib,
    Label,
    Odomentry,
}

impl DataKind {
    fn file_ext(&self) -> Option<&str> {
        Some(match self {
            DataKind::ImageSeq => return None,
            DataKind::VelodyneSeq => return None,
            DataKind::Calib => "txt",
            DataKind::Label => "txt",
            DataKind::Odomentry => "txt",
        })
    }
}

#[derive(Debug, Clone)]
pub struct Sample {
    kind: DataKind,
    path: PathBuf,
}

impl Sample {
    pub fn kind(&self) -> DataKind {
        self.kind
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn data(&self) -> Result<SampleData, Error> {
        SampleData::open(self.kind, &self.path)
    }
}

#[derive(Debug, Clone)]
pub enum SampleData {
    ImageSeq(ImageSeq),
    VelodyneSeq(VelodyneSeq),
    Calib(Box<Calibration>),
    Label(Vec<Label>),
    Odomentry(Vec<Oxts>),
}

impl SampleData {
    fn open<P>(kind: DataKind, path: P) -> Result<SampleData, Error>
    where
        P: AsRef<Path>,
    {
        let sample = match kind {
            DataKind::ImageSeq => {
                let dir = path.as_ref();
                let num_files = super::probe_max_frames(dir, 6, Some("png"));

                SampleData::ImageSeq(ImageSeq {
                    dir: dir.to_owned(),
                    seq_len: num_files,
                })
            }
            DataKind::VelodyneSeq => {
                let dir = path.as_ref();
                let num_files = super::probe_max_frames(dir, 6, Some("bin"));

                SampleData::VelodyneSeq(VelodyneSeq {
                    dir: dir.to_owned(),
                    seq_len: num_files,
                })
            }
            DataKind::Calib => {
                let calib = Calibration::from_path(path)?;
                SampleData::Calib(Box::new(calib))
            }
            DataKind::Label => {
                let labels = Label::vec_from_path(path)?;
                SampleData::Label(labels)
            }
            DataKind::Odomentry => {
                let oxts = Oxts::vec_from_path(path)?;
                SampleData::Odomentry(oxts)
            }
        };

        Ok(sample)
    }
}

#[derive(Debug, Clone)]
pub struct ImageSeq {
    dir: PathBuf,
    seq_len: usize,
}

impl ImageSeq {
    pub fn get(&self, seq_idx: usize) -> Result<Option<DynamicImage>, Error> {
        if seq_idx >= self.seq_len {
            return Ok(None);
        }

        let path = self.dir.join(format!("{seq_idx:06}.png"));
        let image = image::io::Reader::open(path)?.decode()?;
        Ok(Some(image))
    }

    pub fn image_iter(&self) -> impl Iterator<Item = Result<DynamicImage, Error>> + '_ {
        (0..self.seq_len).map(|seq_idx| {
            let path = self.dir.join(format!("{seq_idx:06}.png"));
            let image = image::io::Reader::open(path)?.decode()?;
            Ok(image)
        })
    }

    pub fn seq_len(&self) -> usize {
        self.seq_len
    }
}

#[derive(Debug, Clone)]
pub struct VelodyneSeq {
    dir: PathBuf,
    seq_len: usize,
}

impl VelodyneSeq {
    pub fn get(&self, seq_idx: usize) -> Result<Option<PointCloud>, Error> {
        if seq_idx >= self.seq_len {
            return Ok(None);
        }

        let path = self.dir.join(format!("{seq_idx:06}.bin"));
        let pcd = PointCloud::from_path(path)?;
        Ok(Some(pcd))
    }

    pub fn point_cloud_iter(&self) -> impl Iterator<Item = Result<PointCloud, Error>> + '_ {
        (0..self.seq_len).map(|seq_idx| {
            let path = self.dir.join(format!("{seq_idx:06}.bin"));
            let image = PointCloud::from_path(path)?;
            Ok(image)
        })
    }

    pub fn seq_len(&self) -> usize {
        self.seq_len
    }
}

fn create_file_name(frame_idx: usize, ext: Option<&str>) -> String {
    match ext {
        Some(ext) => format!("{frame_idx:04}.{ext}"),
        None => format!("{frame_idx:04}"),
    }
}
