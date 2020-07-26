use std::cell::Cell;

pub struct Cachable<T> {
    timestamp: Cell<u64>,
    cached: T
}

impl<T: Clone> Cachable<T> {
    pub fn new(cached: T) -> Self {
        Cachable {
            timestamp: Cell::new(time_util::now()),
            cached
        }
    }

    pub fn get_cached(&self) -> T {
        self.timestamp.set(time_util::now());
        self.cached.clone()
    }
}

unsafe impl<T> Send for Cachable<T> {}
unsafe impl<T> Sync for Cachable<T> {}