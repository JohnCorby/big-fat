use crate::*;
use rayon::prelude::*;
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

fn sum_job(result: &mut AudioResult, readers: Vec<AudioReader>, info: &PollInfo) {
    // read and sum each entire reader, writing to result
    let samples: Vec<f32>;
    time!({
        samples = IntoParallelIterator::into_par_iter(readers)
            .fold(
                || vec![],
                |mut samples, reader| {
                    for (index, sample) in reader.enumerate() {
                        if index >= samples.len() {
                            samples.resize(index + 1, 0.0);
                        }
                        samples[index] += sample;
                        info.samples_done.store(samples.len(), Relaxed);
                    }
                    info.readers_left.fetch_sub(1, Relaxed);
                    samples
                },
            )
            .flatten()
            .collect();
    });

    result.write(samples).unwrap();
}
