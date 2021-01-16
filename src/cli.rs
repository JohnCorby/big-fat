use crate::late_init::LateInit;
use clap::{App, Arg};
use std::path::PathBuf;
use std::time::Duration;

pub static IN_DIR: LateInit<PathBuf> = LateInit::new();
pub static OUT_FILE: LateInit<PathBuf> = LateInit::new();

pub static RECURSE: LateInit<bool> = LateInit::new();

pub static CHANNELS: LateInit<u16> = LateInit::new();
pub static SAMPLE_RATE: LateInit<u32> = LateInit::new();

pub static POLL_DELAY: LateInit<Duration> = LateInit::new();

/// ive tuned this and this number seems to be fastest
pub const CHUNK_SIZE: usize = (1e5 as usize).next_power_of_two();

pub fn parse() {
    let matches = App::new("big fat")
        .about("makes the big noise")
        .arg(
            Arg::with_name("in dir")
                .help("the input directory")
                .required(true)
        )
        .arg(
            Arg::with_name("out file")
                .help("the output file")
                .required(true)
        )
        .arg(
            Arg::with_name("channels")
                .help("the number of channels the input files are required to have, and that the output file will have")
                .long("channels")
                .short("c")
                .default_value("2")
        )
        .arg(
            Arg::with_name("sample rate")
                .help("the sample rate that the input files will be converted to, and that the output file will have")
                .long("sample-rate")
                .short("s")
                .default_value("44100")
        )
        .arg(
            Arg::with_name("poll delay")
                .help("the frequency (in seconds, can be decimal) that the summing info messages will print")
                .long("poll-delay")
                .short("p")
                .default_value(".5")
        )
        .arg(
            Arg::with_name("recurse")
                .help("whether to visit children of in dir recursively, or just the top level files")
                .long("recurse")
                .short("r")
        )
        .get_matches();

    let in_dir: PathBuf = matches.value_of("in dir").unwrap().into();
    assert!(in_dir.exists(), "in dir doesnt exist");
    assert!(in_dir.is_dir(), "in dir must be a directory");
    IN_DIR.init(in_dir);

    let mut out_file: PathBuf = matches.value_of("out file").unwrap().into();
    out_file = out_file.with_extension("wav");
    OUT_FILE.init(out_file);

    CHANNELS.init(matches.value_of("channels").unwrap().parse().unwrap());
    SAMPLE_RATE.init(matches.value_of("sample rate").unwrap().parse().unwrap());

    let poll_delay: f32 = matches.value_of("poll delay").unwrap().parse().unwrap();
    let poll_delay = Duration::from_secs_f32(poll_delay);
    POLL_DELAY.init(poll_delay);

    RECURSE.init(matches.is_present("recurse"));
}
