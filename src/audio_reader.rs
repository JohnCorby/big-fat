//! thingy for reading audio files

use crate::*;
use rodio::source::UniformSourceIterator;
use rodio::{Decoder, Source};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

type Iter = UniformSourceIterator<Decoder<BufReader<File>>, f32>;
pub struct AudioReader {
    iter: Iter,
    at_eof: bool,
    path: PathBuf,
}

impl AudioReader {
    pub fn open(path: &Path) -> Result<Self, String> {
        let file = File::open(&path).map_err(|e| format!("error opening file: {}", e))?;
        let reader = BufReader::new(file);
        let decoder =
            Decoder::new(reader).map_err(|e| format!("error constructing decoder: {}", e))?;

        let channels = decoder.channels();
        let iter = UniformSourceIterator::new(decoder, channels, SAMPLE_RATE);

        try_assert!(
            iter.channels() == CHANNELS,
            "must have {} channels (instead of {})",
            CHANNELS,
            iter.channels()
        );
        try_assert!(
            iter.sample_rate() == SAMPLE_RATE,
            "must have sample rate of {} (instead of {})",
            SAMPLE_RATE,
            iter.sample_rate()
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

impl Debug for AudioReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioReader")
            .field("name", &file_name(&self.path))
            .field("at_eof", &self.at_eof)
            .finish()
    }
}
