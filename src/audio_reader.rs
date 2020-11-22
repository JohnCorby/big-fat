use rodio::source::SamplesConverter;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

type Sample = f32;
pub trait AudioReader: Iterator<Item = Sample> {
    fn open(path: &Path) -> Self;
}

pub struct RodioReader {
    samples_iter: SamplesConverter<Decoder<BufReader<File>>, Sample>,
}
impl AudioReader for RodioReader {
    fn open(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader).unwrap();
        let samples_iter = decoder.convert_samples();
        RodioReader { samples_iter }
    }
}
impl Iterator for RodioReader {
    type Item = Sample;
    fn next(&mut self) -> Option<Self::Item> {
        self.samples_iter.next()
    }
}
