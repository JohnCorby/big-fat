use crate::*;
// use rayon::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub fn make_result(result: &mut AudioResult, readers: Vec<AudioReader>) {
    let info = PollInfo {
        readers_left: AtomicUsize::new(readers.len()),
        chunks_done: AtomicUsize::new(0),
        current_reader: AtomicUsize::new(0),
    };
    rayon::scope(|s| {
        s.spawn(|_| poll_job(&info));
        sum_job(result, readers, &info);
    });
}

#[derive(Debug)]
struct PollInfo {
    readers_left: AtomicUsize,
    chunks_done: AtomicUsize,
    current_reader: AtomicUsize,
}

fn poll_job(info: &PollInfo) {
    while info.readers_left.load(Relaxed) > 0 {
        println!("{:?}", info);
        std::thread::sleep(POLL_DELAY);
    }
}

fn sum_job(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
    let mut chunk = vec![0.0; CHUNK_SIZE];
    while !readers.is_empty() {
        // read and sum
        chunk = readers
            .iter_mut()
            .enumerate()
            .fold(chunk, |mut chunk, (reader_index, reader)| {
                for (chunk_index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                    chunk[chunk_index] += sample;
                }
                info.current_reader.store(reader_index, Relaxed);
                if reader.at_eof() {
                    info.readers_left.fetch_sub(1, Relaxed);
                }
                chunk
            });
        info.chunks_done.fetch_add(1, Relaxed);

        // remove done
        readers.drain_filter(|reader| reader.at_eof());

        // write to result, resetting the chunk
        result.flush(&mut chunk).unwrap();
    }
}
