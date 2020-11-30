#![allow(unused)]

mod audio_reader;
mod audio_result;
mod util;

use anyhow::Error;
use anyhow::*;
use audio_reader::AudioReader;
use audio_result::AudioResult;
use rayon::prelude::*;
use rayon::vec::IntoIter;
use std::mem::size_of;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use util::*;
use walkdir::WalkDir;

// config
const IN_DIR: &str =
    r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay";
pub const CHANNELS: u16 = 2;
pub const SAMPLE_RATE: u32 = 44100;
pub const OUT_FILE: &str = r".\bruh.wav";
pub const POLL_EVERY: u32 = 10;

fn main() -> Result<()> {
    // read all paths recursively, ignoring errors
    println!("OPENING FILES");
    let mut readers = WalkDir::new(IN_DIR)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .filter_map(|path| match AudioReader::open(&path) {
            Ok(reader) => Some(reader),
            Err(err) => {
                println!(
                    "error opening {}: {:?}",
                    path.file_name().unwrap().to_str().unwrap(),
                    err
                );
                None
            }
        })
        .collect::<Vec<_>>();

    // go thru every sample of every file and add it to the result
    println!("SUMMING FILES");
    let mut result = AudioResult::new().context("error constructing audio result")?;
    pause();
    time!({
        let mut sample_index = 0;
        let mut to_remove = Vec::with_capacity(readers.len());
        while !readers.is_empty() {
            let mut result_sample = 0.0;
            for (reader_index, reader) in readers.iter_mut().enumerate() {
                match reader.next() {
                    Some(sample) => result_sample += sample,
                    None => to_remove.push(reader_index),
                }
            }
            result
                .push(result_sample)
                .context("error pushing result sample")?;

            while let Some(reader_index) = to_remove.pop() {
                readers.remove(reader_index);
            }

            if sample_index % (SAMPLE_RATE * POLL_EVERY) == 0 {
                println!(
                    "{:?} in, {} readers left",
                    Duration::from_secs((sample_index / SAMPLE_RATE) as u64),
                    readers.len()
                );
            }

            sample_index += 1;
        }
    });
    pause();

    // save the result
    println!("SAVING RESULT");
    result.save().context("error saving audio result")?;

    println!("DONE!");
    Ok(())
}
