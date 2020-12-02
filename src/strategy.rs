use crate::poll_info::PollInfo;
use crate::*;
use std::cmp::Ordering;

/// a way to sum the readers into the result
pub trait Strategy {
    fn execute(result: &mut AudioResult, readers: Vec<AudioReader>, info: &PollInfo);
}

/// 1 reader at a time, one chunk at a time
pub struct Strategy1;
impl Strategy for Strategy1 {
    fn execute(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
        while !readers.is_empty() {
            // read and sum
            let chunk = par_fold(
                &mut readers,
                || vec![0.0; CHUNK_SIZE],
                |mut chunk, reader| {
                    for (index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                        chunk[index] += sample;
                    }
                    if reader.at_eof() {
                        info.reader_done();
                    }
                    chunk
                },
                |ls, rs| ls.into_iter().zip(rs).map(|(l, r)| l + r).collect(),
            );
            info.chunk_done();

            // remove done
            readers.drain_filter(|reader| reader.at_eof());

            // write to result
            for sample in chunk {
                result.write(sample).unwrap();
            }
        }
    }
}

/// 1 reader at a time, all the way thru
pub struct Strategy2;
impl Strategy for Strategy2 {
    fn execute(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
        let samples = par_fold(
            &mut readers,
            Vec::new,
            |mut samples, reader| {
                for (index, sample) in reader.enumerate() {
                    if index >= samples.len() {
                        samples.resize(index + 1, 0.0);
                    }
                    samples[index] = sample;
                }
                info.reader_done();
                samples
            },
            |mut ls, mut rs| {
                // resize the smaller one to the bigger one
                match ls.len().cmp(&rs.len()) {
                    Ordering::Less => ls.resize(rs.len(), 0.0),
                    Ordering::Greater => rs.resize(ls.len(), 0.0),
                    _ => {}
                };
                ls.into_iter().zip(rs).map(|(l, r)| l + r).collect()
            },
        );
        info.chunk_done();

        for sample in samples {
            result.write(sample).unwrap();
        }
    }
}
