//! thingy for reading audio files

use crate::*;
use anyhow::*;
use rodio::source::SamplesConverter;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

type SampleIter = SamplesConverter<Decoder<BufReader<File>>, f32>;

pub struct AudioReader {
    sample_iter: SampleIter,
    reached_none: bool,
}

impl AudioReader {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path).context("error opening file")?;
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
            reached_none: false,
        })
    }

    pub fn reached_none(&self) -> bool {
        self.reached_none
    }
}

impl Iterator for AudioReader {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.sample_iter.next();
        self.reached_none = next.is_none();
        next
    }
}
