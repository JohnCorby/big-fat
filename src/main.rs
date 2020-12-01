#![feature(drain_filter)]
#![feature(slice_fill)]
#![feature(iterator_fold_self)]
#![allow(dead_code)]

mod audio_reader;
mod audio_result;
mod main_task;
mod util;

use anyhow::*;
use audio_reader::AudioReader;
use audio_result::AudioResult;
use std::time::Duration;
// use util::*;
use main_task::make_result;
use walkdir::WalkDir;

// config
const IN_DIR: &str = r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay\vgm server (ie parker)";
pub const OUT_FILE: &str = r".\bruh.wav";

pub const CHANNELS: u16 = 2;
pub const SAMPLE_RATE: u32 = 44100;

pub const CHUNK_SIZE: usize = (1e9 as usize / 4).next_power_of_two();
pub const POLL_DELAY: Duration = Duration::from_millis(1000 / 3);

fn main() -> Result<()> {
    // read all paths recursively, ignoring errors
    println!("OPENING FILES");
    let readers = open_readers();

    // go thru every sample of every file and add it to the result
    println!("SUMMING FILES");
    let mut result = AudioResult::new().context("error constructing audio result")?;
    // pause();
    time!({ make_result(&mut result, readers) });
    // pause();

    // save the result
    println!("SAVING RESULT");
    result.save().context("error saving audio result")?;

    println!("DONE!");
    Ok(())
}

fn open_readers() -> Vec<AudioReader> {
    WalkDir::new(IN_DIR)
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
        .collect()
}
