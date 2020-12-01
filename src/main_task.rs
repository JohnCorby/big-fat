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
    while !readers.is_empty() {
        // get a sum of the samples
        let sample = readers.iter_mut().fold(0.0, |sample, reader| {
            sample + reader.next().unwrap_or_default()
        });
        info.samples_done.fetch_add(1, Relaxed);

        // remove done
        readers.drain_filter(|reader| reader.at_eof());
        info.readers_left.store(readers.len(), Relaxed);

        // write the sample
        result.write(sample).unwrap();
    }
}
