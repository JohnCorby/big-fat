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
    loop {
        // break if done
        if info.readers_left.load(Relaxed) == 0 {
            break;
        }

        println!("{:?}", info);
        std::thread::sleep(POLL_EVERY);
    }
}

// type ReaderLock<'a> = Mutex<&'a mut AudioReader>;
fn sum_job(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
    let mut chunk = vec![0.0; CHUNK_SIZE];
    while info.readers_left.load(Relaxed) > 0 {
        // read and sum
        for reader in readers.iter_mut() {
            let samples_done = info.samples_done.load(Relaxed);
            for (chunk_index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                chunk[chunk_index] += sample;
                info.samples_done.store(samples_done + chunk_index, Relaxed);
            }
            if reader.at_eof() {
                info.readers_left.fetch_sub(1, Relaxed);
            }
        }

        // write to result, resetting the chunk
        result.flush(&mut chunk).unwrap();
    }
}
