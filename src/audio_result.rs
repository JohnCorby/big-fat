//! thingy for storing the sum of the audio files

use crate::{CHANNELS, OUT_FILE, SAMPLE_RATE};
use anyhow::*;
use hound::{SampleFormat, WavSpec, WavWriter};

pub struct AudioResult {
    samples: Vec<f32>,
}

impl AudioResult {
    pub fn new() -> Self {
        AudioResult { samples: vec![] }
    }

    pub fn add(&mut self, index: usize, sample: f32) {
        if index >= self.samples.len() {
            self.samples.resize(index + 1, Default::default());
        }

        self.samples[index] += sample;
    }

    pub fn save(&mut self) -> Result<()> {
        let spec = WavSpec {
            channels: CHANNELS as u16,
            sample_rate: SAMPLE_RATE as u32,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let mut writer = WavWriter::create(OUT_FILE, spec).context("error creating wav writer")?;
        for sample in &self.samples {
            writer
                .write_sample(*sample)
                .context("error writing sample")?;
        }
        writer.finalize().context("error finalizing wav writer")?;
        Ok(())
    }
}
