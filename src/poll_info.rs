use crate::POLL_DELAY;
use std::fmt::{Display, Formatter, Result};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub struct PollInfo {
    readers_left: AtomicUsize,
    iterations_done: AtomicUsize,
}

impl PollInfo {
    pub fn new(num_readers: usize) -> Self {
        Self {
            readers_left: AtomicUsize::new(num_readers),
            iterations_done: Default::default(),
        }
    }
    pub fn reader_done(&self) {
        self.readers_left.fetch_sub(1, Relaxed);
    }
    pub fn iteration_done(&self) {
        self.iterations_done.fetch_add(1, Relaxed);
    }
}

impl Display for PollInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?} readers left\t\t{:?} iterations done",
            self.readers_left, self.iterations_done
        )
    }
}

pub fn poll_job(info: &PollInfo) {
    while info.readers_left.load(Relaxed) > 0 {
        println!("{}", info);
        std::thread::sleep(POLL_DELAY);
    }
}
