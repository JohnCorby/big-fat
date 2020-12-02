//! thingy for storing the sum of the audio files

use crate::*;
use anyhow::*;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

type Writer = WavWriter<BufWriter<File>>;

pub struct AudioResult {
    writer: Writer,
}

impl AudioResult {
    pub fn new() -> Result<Self> {
        let spec = WavSpec {
            channels: CHANNELS,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let writer = Writer::create(OUT_FILE, spec).context("error creating wav writer")?;
        Ok(AudioResult { writer })
    }

    pub fn write(&mut self, sample: f32) -> Result<()> {
        self.writer
            .write_sample(sample)
            .context("error writing sample")
    }

    pub fn save(self) -> Result<()> {
        self.writer
            .finalize()
            .context("error finalizing wav writer")
    }
}

impl Debug for AudioResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioResult")
            .field("name", &file_name(Path::new(OUT_FILE)))
            .finish()
    }
}
