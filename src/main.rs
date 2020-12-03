#![feature(drain_filter)]
#![allow(dead_code)]

mod audio_reader;
mod audio_result;
mod poll_info;
mod strategy;
mod util;

use crate::poll_info::{poll_job, PollInfo};
use crate::strategy::*;
use audio_reader::AudioReader;
use audio_result::AudioResult;
use std::time::Duration;
use util::*;
use walkdir::WalkDir;

// config
const IN_DIR: &str =
    r"D:\OneDrive - Lake Washington School District\Everything Else\gay\sound is gay";
pub const OUT_FILE: &str = r".\bruh.wav";

pub const CHANNELS: u16 = 2;
pub const SAMPLE_RATE: u32 = 44100;

pub const POLL_DELAY: Duration = Duration::from_millis(1000 / 3);

fn main() {
    rayon::ThreadPoolBuilder::new()
        .thread_name(|i| format!("rayon thread {}", i))
        .build_global()
        .unwrap();

    // read all paths recursively, ignoring errors
    println!("OPENING FILES");
    let readers;
    println!("DONE IN {:?}", time!({ readers = open_readers() }));

    print!("\n\n\n");

    // go thru every sample of every file and add it to the result
    println!("SUMMING FILES");
    let mut result = AudioResult::new();
    // pause();
    println!("DONE IN {:?}", time!({ sum(&mut result, readers) }));
    // pause();

    print!("\n\n\n");

    // save the result
    println!("SAVING RESULT");
    println!("DONE IN {:?}", time!({ result.save() }));
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
                println!("error opening {}: {:?}", file_name(&path), err);
                None
            }
        })
        // .flat_map(|reader| (0..10).map(move |_| reader.clone())) // artificial lengthening
        .collect()
}

fn sum(result: &mut AudioResult, readers: Vec<AudioReader>) {
    let info = PollInfo::new(readers.len());
    rayon::join(
        || poll_job(&info),
        || {
            Strategy3::execute(result, readers, &info);
            info.done();
        },
    );
}
