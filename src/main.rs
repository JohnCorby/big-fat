mod audio_reader;
mod audio_result;
mod util;

use crate::audio_reader::AudioReader;
use crate::audio_result::AudioResult;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

// config
const IN_DIR: &str = r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay\vgm server (ie parker)";
pub const CHANNELS: u16 = 2;
pub const SAMPLE_RATE: u32 = 44100;
pub const OUT_FILE: &str = r".\bruh.wav";

fn main() {
    // read all paths recursively, ignoring errors
    let paths = WalkDir::new(IN_DIR)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    // artificially extend by duplicating N times
    // const N: usize = 3;
    // let total = paths.len();
    // let paths = paths
    //     .into_iter()
    //     .cycle()
    //     .take(total * N)
    //     .collect::<Vec<_>>();

    let total = paths.len();
    let count = AtomicUsize::new(1);

    let iter: rayon::vec::IntoIter<_> = paths.into_par_iter();
    // let iter = paths.into_iter();
    let mut audio_results = vec![];
    util::time(|| {
        audio_results = iter
            .fold(
                || AudioResult::new(),
                |mut audio_result, path| {
                    let prefix = format!(
                        "{}/{} - {:?}",
                        count.fetch_add(1, Ordering::Relaxed),
                        total,
                        path.file_name().unwrap()
                    );

                    let reader = AudioReader::open(&path);
                    if reader.is_err() {
                        println!("{} - error: {:?}", prefix, reader.err().unwrap());
                        return audio_result;
                    }
                    let reader = reader.unwrap();

                    let mut num_samples = 0;
                    for (index, sample) in reader.enumerate() {
                        audio_result.add(index, sample);
                        num_samples = index;
                    }

                    println!(
                        "{} - {} mb",
                        prefix,
                        (num_samples * std::mem::size_of::<f32>()) as f32 / 1_000_000f32
                    );

                    audio_result
                },
            )
            .collect::<Vec<_>>()
    });
    util::pause();
}
