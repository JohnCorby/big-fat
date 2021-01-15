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

    let in_dir: PathBuf = matches.value_of("in dir").unwrap().into();
    assert!(in_dir.exists(), "in dir doesnt exist");
    assert!(in_dir.is_dir(), "in dir must be a directory");
    IN_DIR.init(in_dir);

    let mut out_file: PathBuf = matches.value_of("out file").unwrap().into();
    out_file = out_file.with_extension("wav");
    OUT_FILE.init(out_file);
}
