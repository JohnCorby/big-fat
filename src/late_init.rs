use std::lazy::SyncOnceCell;
use std::ops::Deref;

pub struct LateInit<T>(SyncOnceCell<T>);
impl<T> LateInit<T> {
    pub const fn new() -> Self {
        Self(SyncOnceCell::new())
    }
    pub fn init(&self, value: T) {
        assert!(self.0.get().is_none(), "late init already initialized");
        let _ = self.0.set(value);
    }
}
impl<T> Deref for LateInit<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.get().is_some(), "late init not initialized");
        self.0.get().unwrap()
    }
}
