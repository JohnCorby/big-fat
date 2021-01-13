use crate::late_init::LateInit;
use clap::{App, Arg};
use std::path::PathBuf;

pub static IN_DIR: LateInit<PathBuf> = LateInit::new();
pub static OUT_FILE: LateInit<PathBuf> = LateInit::new();

pub fn parse() {
    let matches = App::new("big fat")
        .about("makes the big noise")
        .arg(
            Arg::with_name("in dir")
                .help("the input directory (traversed recursively)")
                .required(true),
        )
        .arg(
            Arg::with_name("out file")
                .help("the output file")
                .required(true),
        )
        .get_matches();

    IN_DIR.init(matches.value_of("in dir").unwrap().into());
    OUT_FILE.init(matches.value_of("out file").unwrap().into());
}
