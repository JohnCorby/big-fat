//! thingy for reading audio files

use crate::*;
use anyhow::*;
use rodio::source::SamplesConverter;
use rodio::{Decoder, Source};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

type SampleIter = SamplesConverter<Decoder<BufReader<File>>, f32>;

pub struct AudioReader {
    sample_iter: SampleIter,
    at_eof: bool,
    path: PathBuf,
}

impl AudioReader {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(&path).context("error opening file")?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).context("error constructing decoder")?;
        let sample_iter = decoder.convert_samples();

        ensure!(
            sample_iter.channels() == CHANNELS,
            "must have {} channels",
            CHANNELS
        );
        ensure!(
            sample_iter.sample_rate() == SAMPLE_RATE,
            "must have sample rate of {}",
            SAMPLE_RATE
        );

        Ok(AudioReader {
            sample_iter,
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
        let next = self.sample_iter.next();
        self.at_eof = next.is_none();
        next
    }
}

impl Display for AudioReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "audio reader ({})", file_name(&self.path))
    }
}
