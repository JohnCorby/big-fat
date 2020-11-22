//! thingy for storing the sum of the audio files

use crate::{CHANNELS, OUT_FILE, SAMPLE_RATE};
use hound::{SampleFormat, WavSpec};

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

    pub fn save(&mut self) {
        let spec = WavSpec {
            channels: CHANNELS,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let mut writer =
            hound::WavWriter::create(OUT_FILE, spec).expect("error creating wav writer");
        for sample in &self.samples {
            writer.write_sample(*sample).expect("error writing sample");
        }
        writer.finalize().expect("error finalizing wav writer");
    }
}
