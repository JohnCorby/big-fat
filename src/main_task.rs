use crate::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub fn make_result(result: &mut AudioResult, readers: Vec<AudioReader>) {
    let info = PollInfo {
        samples_done: AtomicUsize::new(0),
        readers_left: AtomicUsize::new(readers.len()),
    };
    rayon::scope(|s| {
        s.spawn(|_| poll_job(&info));
        sum_job(result, readers, &info);
    });
}

#[derive(Debug)]
struct PollInfo {
    samples_done: AtomicUsize,
    readers_left: AtomicUsize,
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
        readers.iter_mut().fold(&mut chunk, |chunk, reader| {
            let samples_done = info.samples_done.load(Relaxed);
            for (chunk_index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                chunk[chunk_index] += sample;
                info.samples_done
                    .store(samples_done + chunk_index + 1, Relaxed);
            }
            if reader.at_eof() {
                info.readers_left.fetch_sub(1, Relaxed);
            }
            chunk
        });

        // remove done
        readers.drain_filter(|reader| reader.at_eof());

        // write to result, resetting the chunk
        result.flush(&mut chunk).unwrap();
    }
}
