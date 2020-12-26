//! thingy for reading audio files

use crate::*;
use rodio::source::SamplesConverter;
use rodio::{Decoder, Source};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

type Iter = SamplesConverter<Decoder<BufReader<File>>, f32>;
pub struct AudioReader {
    iter: Iter,
    at_eof: bool,
    path: PathBuf,
}

impl AudioReader {
    pub fn open(path: &Path) -> Result<Self, String> {
        let file = File::open(&path).map_err(|_| "error opening file")?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).map_err(|_| "error constructing decoder")?;
        let iter = decoder.convert_samples();

        try_assert!(
            iter.channels() == CHANNELS,
            "must have {} channels",
            CHANNELS
        );
        try_assert!(
            iter.sample_rate() == SAMPLE_RATE,
            "must have sample rate of {}",
            SAMPLE_RATE
        );

        Ok(Self {
            iter,
            at_eof: false,
            path: path.into(),
        })
    }

    pub fn at_eof(&self) -> bool {
        self.at_eof
    }
}

impl Iterator for AudioReader {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        self.at_eof = next.is_none();
        next
    }
}

/// todo unnecessary and shit
impl Clone for AudioReader {
    fn clone(&self) -> Self {
        Self::open(&self.path).unwrap()
    }
}

impl Debug for AudioReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioReader")
            .field("name", &file_name(&self.path))
            .field("at_eof", &self.at_eof)
            .finish()
    }
}
