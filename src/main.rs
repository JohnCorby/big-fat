#![feature(drain_filter)]
#![feature(once_cell)]
#![feature(panic_info_message)]
#![allow(dead_code)]

mod audio_reader;
mod audio_result;
mod cli;
mod late_init;
mod poll_info;
mod util;

use audio_reader::AudioReader;
use audio_result::AudioResult;
use cli::IN_DIR;
use poll_info::{poll_job, PollInfo};
use rayon::prelude::*;
use std::time::Duration;
use util::*;
use walkdir::WalkDir;

pub const CHANNELS: u16 = 2;
pub const SAMPLE_RATE: u32 = 44100;

pub const POLL_DELAY: Duration = Duration::from_millis(1000 / 3);
/// ive tuned this and this number seems to be fastest
const CHUNK_SIZE: usize = (1e5 as usize).next_power_of_two();

fn main() {
    // nicer error messages
    std::panic::set_hook(Box::new(|info| {
        let error = if let Some(&message) = info.message() {
            format!("{}", message)
        } else if let Some(&payload) = info.payload().downcast_ref::<&'static str>() {
            payload.to_string()
        } else {
            "[unknown error]".to_string()
        };
        println!("error: {}", error);
    }));

    cli::parse();

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
    let mut good = 0usize;
    let mut total = 0usize;
    let readers = WalkDir::new(&*IN_DIR)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.into_path())
        .filter(|path| path.is_file())
        .filter(|path| matches!(file_extension(&path), "wav" | "flac" | "mp3" | "ogg"))
        .inspect(|_| total += 1)
        .filter_map(|path| match AudioReader::open(&path) {
            Ok(reader) => {
                good += 1;
                Some(reader)
            }
            Err(err) => {
                println!("error opening {}: {}", file_name(&path), err);
                None
            }
        })
        .collect();
    println!("opened {} out of {} readers", good, total);
    readers
}

fn sum(result: &mut AudioResult, readers: Vec<AudioReader>) {
    let info = PollInfo::new(readers.len());
    rayon::join(
        || poll_job(&info),
        || {
            sum_job(result, readers, &info);
            info.done();
        },
    );
}

fn sum_job(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
    while !readers.is_empty() {
        // read and sum
        let chunk = par_iter(&mut readers)
            .map(|reader| reader.take(CHUNK_SIZE).collect())
            .reduce(
                || vec![0.0; CHUNK_SIZE],
                |a: Vec<f32>, b: Vec<f32>| {
                    a.into_iter()
                        .zip(b.into_iter())
                        .map(|(a, b)| a + b)
                        .collect()
                },
            );

        // write to result
        for sample in chunk {
            result.write(sample);
        }

        // remove done
        readers
            .drain_filter(|reader| reader.at_eof())
            .for_each(|_| info.reader_done());

        info.iteration_done();
    }
}
