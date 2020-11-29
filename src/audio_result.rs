//! thingy for storing the sum of the audio files

use crate::{CHANNELS, OUT_FILE, SAMPLE_RATE};
use anyhow::*;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::fs::File;
use std::io::BufWriter;

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

    pub fn push(&mut self, sample: f32) -> Result<()> {
        self.writer
            .write_sample(sample)
            .context("error writing sample")
    }

    pub fn save(mut self) -> Result<()> {
        // fixme make this a real fix instead of just padding lmao
        let desired_num_samples = self.writer.duration() * CHANNELS as u32 + CHANNELS as u32;
        while self.writer.len() < desired_num_samples {
            self.push(0.0);
        }

        self.writer
            .finalize()
            .context("error finalizing wav writer")
    }
}
