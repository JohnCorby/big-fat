use crate::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Mutex;

pub fn main_task(result: &mut AudioResult, readers: &mut [AudioReader]) {
    let info = PollInfo {
        samples_done: AtomicUsize::new(0),
        readers_left: AtomicUsize::new(readers.len()),
    };
    let readers = readers.iter_mut().map(ReaderLock::new).collect::<Vec<_>>();
    rayon::scope(|s| {
        s.spawn(|_| sum_job(result, &readers, &info));
        s.spawn(|_| poll_job(&info));
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

type ReaderLock<'a> = Mutex<&'a mut AudioReader>;
fn sum_job(result: &mut AudioResult, readers: &[ReaderLock], info: &PollInfo) {
    let mut chunk = vec![0.0; CHUNK_SIZE];
    loop {
        // break if done
        if info.readers_left.load(Relaxed) == 0 {
            break;
        }

        // read and sum
        chunk.fill(0.0);
        for reader in readers.iter() {
            let mut reader = reader.lock().unwrap();
            for (chunk_index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                chunk[chunk_index] += sample;
            }
        }
        info.samples_done.fetch_add(CHUNK_SIZE, Relaxed);

        // write to result
        result.push(&chunk).unwrap();

        // track remaining
        info.readers_left.store(
            readers
                .iter()
                .filter(|reader| {
                    let reader = reader.lock().unwrap();
                    !reader.reached_none()
                })
                .count(),
            Relaxed,
        );
    }
}
