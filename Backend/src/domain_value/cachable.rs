use std::sync::RwLock;

pub struct Cachable<T> {
    timestamp: RwLock<u64>,
    cached: T
}

impl<T: Clone> Cachable<T> {
    pub fn new(cached: T) -> Self {
        Cachable {
            timestamp: RwLock::new(time_util::now()),
            cached
        }
    }

    pub fn get_cached(&self) -> T {
        let mut ts = self.timestamp.write().unwrap();
        *ts = time_util::now();
        self.cached.clone()
    }
}