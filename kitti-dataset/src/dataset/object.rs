use crate::{
    common::PointCloud,
    object::{Calibration, Label},
    Error,
};
use image::DynamicImage;
use itertools::Itertools;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ObjectDataset {
    dataset_dir: PathBuf,
    num_frames: usize,
    sub_dirs: HashMap<String, DataKind>,
}

impl ObjectDataset {
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
                    DataKind::Image
                } else if file_name.starts_with("velodyne") {
                    DataKind::Velodyne
                } else if file_name.starts_with("label") {
                    DataKind::Label
                } else if file_name.starts_with("calib") {
                    DataKind::Calib
                } else {
                    skip!();
                };

                Ok(Some((file_name.to_string(), kind)))
            })
            .flatten_ok()
            .try_collect()?;

        let num_frames = match sub_dirs.iter().next() {
            Some((key, kind)) => {
                super::probe_max_frames(&dataset_dir.join(key), 6, Some(kind.file_ext()))
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
    dataset: &'a ObjectDataset,
    frame_idx: usize,
}

impl<'a> Frame<'a> {
    pub fn key(&self, key: &str) -> Option<Sample> {
        let kind = *self.dataset.sub_dirs.get(key)?;
        let path = self.dataset.dataset_dir.join(key).join(format!(
            "{:06}.{}",
            self.frame_idx,
            kind.file_ext()
        ));
        let sample = Sample { kind, path };
        Some(sample)
    }

    pub fn sample_iter(&self) -> impl Iterator<Item = Sample> + '_ {
        self.dataset.sub_dirs.iter().map(|(key, &kind)| {
            let path = self.dataset.dataset_dir.join(key).join(format!(
                "{:06}.{}",
                self.frame_idx,
                kind.file_ext()
            ));
            Sample { kind, path }
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeyEntry<'a> {
    dataset: &'a ObjectDataset,
    key: String,
    kind: DataKind,
}

impl<'a> KeyEntry<'a> {
    pub fn frame(&self, frame_idx: usize) -> Option<Sample> {
        if frame_idx >= self.dataset.num_frames {
            return None;
        }

        let path = self.dataset.dataset_dir.join(&self.key).join(format!(
            "{:06}.{}",
            frame_idx,
            self.kind.file_ext()
        ));

        let sample = Sample {
            kind: self.kind,
            path,
        };
        Some(sample)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataKind {
    Image,
    Velodyne,
    Calib,
    Label,
}

impl DataKind {
    fn file_ext(&self) -> &str {
        match self {
            DataKind::Image => "png",
            DataKind::Velodyne => "bin",
            DataKind::Calib => "txt",
            DataKind::Label => "txt",
        }
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
    Image(Box<DynamicImage>),
    Velodyne(Box<PointCloud>),
    Calib(Box<Calibration>),
    Label(Vec<Label>),
}

impl SampleData {
    pub fn open<P>(kind: DataKind, path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let sample = match kind {
            DataKind::Image => {
                let image = image::io::Reader::open(path)?.decode()?;
                SampleData::Image(Box::new(image))
            }
            DataKind::Velodyne => {
                let pcd = PointCloud::from_path(path)?;
                SampleData::Velodyne(Box::new(pcd))
            }
            DataKind::Calib => {
                let calib = Calibration::from_path(path)?;
                SampleData::Calib(Box::new(calib))
            }
            DataKind::Label => {
                let labels = Label::vec_from_path(path)?;
                SampleData::Label(labels)
            }
        };
        Ok(sample)
    }
}
