mod audio_reader;
mod audio_result;
mod util;

use crate::audio_reader::AudioReader;
use crate::audio_result::AudioResult;
use rayon::prelude::*;
use rayon::vec::IntoIter;
use std::mem::size_of;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

// config
const IN_DIR: &str =
    r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay";
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

    let total = paths.len();
    let num_done = AtomicUsize::new(1);

    let iter: IntoIter<PathBuf> = paths.into_par_iter();
    let mut audio_results = vec![];
    util::time(|| {
        audio_results = iter
            .fold(
                || AudioResult::new(),
                |mut audio_result, path| {
                    let name = path.file_name().unwrap().to_str().unwrap();

                    match AudioReader::open(&path) {
                        Ok(reader) => {
                            // process file and track samples
                            let mut num_samples = 0;
                            for (j, sample) in reader.enumerate() {
                                audio_result.add(j, sample);
                                num_samples = j;
                            }

                            // print bytes size of samples
                            println!(
                                "{} - {} mb",
                                name,
                                (num_samples * size_of::<f32>()) as f32 / 1_000_000f32
                            );
                        }
                        Err(error) => println!("{} - error: {:?}", name, error),
                    }

                    // increment/print done
                    let num_done = num_done.fetch_add(1, Ordering::Relaxed);
                    println!("{}/{}", num_done, total);

                    audio_result
                },
            )
            .collect::<Vec<_>>()
    });
    util::pause();
}
