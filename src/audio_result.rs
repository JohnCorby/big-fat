//! thingy for storing the sum of the audio files

use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::BufWriter;

use hound::{SampleFormat, WavSpec, WavWriter};

use crate::cli::{CHANNELS, OUT_FILE, SAMPLE_RATE};
use crate::*;

type Writer = WavWriter<BufWriter<File>>;
pub struct AudioResult {
    writer: Writer,
}

impl AudioResult {
    pub fn new() -> Self {
        let spec = WavSpec {
            channels: *CHANNELS,
            sample_rate: *SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let writer = Writer::create(&*OUT_FILE, spec).expect("error creating wav writer");
        Self { writer }
    }

    pub fn write(&mut self, sample: f32) {
        self.writer
            .write_sample(sample)
            .expect("error writing sample")
    }

    pub fn save(self) {
        self.writer.finalize().expect("error finalizing wav writer")
    }
}

impl Debug for AudioResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioResult")
            .field("name", &file_name(&OUT_FILE))
            .finish()
    }
}
