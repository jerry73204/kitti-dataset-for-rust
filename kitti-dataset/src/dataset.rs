pub mod object;
pub mod tracking;

use std::{iter, path::Path};

pub use object::ObjectDataset;
pub use tracking::TrackingDataset;

fn probe_max_frames(dir: &Path, width: usize, ext: Option<&str>) -> usize {
    let powers = || iter::successors(Some(1u64), |prev| Some(prev * 2));
    let path = |idx| match ext {
        Some(ext) => dir.join(format!("{idx:0width$}.{ext}")),
        None => dir.join(format!("{idx:0width$}")),
    };

    let mut max = powers().find(|&idx| !path(idx).exists()).unwrap();
    let mut min = max / 2;

    if !path(min).exists() {
        debug_assert_eq!(min, 0);
        return 0;
    }

    while min + 1 < max {
        let med = (min + max) / 2;

        if path(med).exists() {
            min = med;
        } else {
            max = med;
        }
    }

    max as usize
}
