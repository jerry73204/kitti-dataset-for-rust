use anyhow::Result;
use clap::{Parser, ValueEnum};
use indicatif::ProgressBar;
use kitti_dataset::dataset::{ObjectDataset, TrackingDataset};
use rayon::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(long)]
    pub kind: DatasetKind,
    #[clap(long)]
    pub test: bool,
    pub dataset_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
enum DatasetKind {
    Object,
    Tracking,
}

fn main() -> Result<()> {
    let Opts {
        kind,
        test,
        dataset_dir,
    } = Opts::parse();

    match kind {
        DatasetKind::Object => {
            let dataset = ObjectDataset::open(dataset_dir)?;
            let num_frames = dataset.num_frames();
            println!("Found {} frames", num_frames);

            // Print keys
            let mut keys: Vec<_> = dataset.keys().map(|(k, _)| k).collect();
            keys.sort();
            println!("Found keys: {}", keys.join(", "));

            if test {
                let bar = ProgressBar::new(num_frames as u64);

                (0..num_frames).into_par_iter().for_each(|frame_idx| {
                    let frame = dataset.frame(frame_idx).unwrap();

                    for sample in frame.sample_iter() {
                        if let Err(err) = sample.data() {
                            eprintln!("fail to load {}: {err}", sample.path().display());
                        }
                    }

                    bar.inc(1);
                });

                bar.finish();
            }
        }
        DatasetKind::Tracking => {
            let dataset = TrackingDataset::open(dataset_dir)?;
            let num_frames = dataset.num_frames();
            println!("Found {} frames", num_frames);

            // Print keys
            let mut keys: Vec<_> = dataset.keys().map(|(k, _)| k).collect();
            keys.sort();
            println!("Found keys: {}", keys.join(", "));

            use kitti_dataset::dataset::tracking::DataKind;
            let num_seq_keys = dataset
                .keys()
                .filter(|(_, k)| [DataKind::ImageSeq, DataKind::VelodyneSeq].contains(k))
                .count();

            if test {
                println!("Check sequence length for each frame");

                let total_seq_len: usize = dataset
                    .frame_iter()
                    .enumerate()
                    .map(|(frame_idx, frame)| {
                        let seq_len = frame.seq_len().unwrap();
                        println!("    frame {frame_idx}: {seq_len}");
                        seq_len
                    })
                    .sum();
                let total_seq_samples = num_seq_keys * total_seq_len;

                let bar = ProgressBar::new(total_seq_samples as u64);

                (0..num_frames).into_par_iter().for_each(|frame_idx| {
                    let frame = dataset.frame(frame_idx).unwrap();

                    for sample in frame.sample_iter() {
                        match sample.data() {
                            Ok(data) => {
                                use kitti_dataset::dataset::tracking::SampleData;

                                match data {
                                    SampleData::ImageSeq(seq) => {
                                        for image in seq.image_iter() {
                                            if let Err(err) = image {
                                                eprintln!(
                                                    "fail to load {}: {err}",
                                                    sample.path().display()
                                                );
                                            }

                                            bar.inc(1);
                                        }
                                    }
                                    SampleData::VelodyneSeq(seq) => {
                                        for pcd in seq.point_cloud_iter() {
                                            if let Err(err) = pcd {
                                                eprintln!(
                                                    "fail to load {}: {err}",
                                                    sample.path().display()
                                                );
                                            }

                                            bar.inc(1);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Err(err) => {
                                eprintln!("fail to load {}: {err}", sample.path().display());
                            }
                        }
                    }
                });

                bar.finish();
            }
        }
    }

    Ok(())
}
