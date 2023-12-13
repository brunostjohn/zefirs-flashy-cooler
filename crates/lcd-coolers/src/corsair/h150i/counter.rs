use std::time::{Duration, SystemTime};

pub(crate) struct Counter {
    frequency: Duration,
    time: SystemTime
}

impl Counter {
    pub fn new(frequency_ms: u64) -> Self {
        Self {
            frequency: Duration::from_millis(frequency_ms),
            time: SystemTime::now(),
        }
    }

    #[inline(always)]
    pub fn check_time(&mut self) -> bool {
        if self.time.try_elapsed(self.frequency) {
            self.time = SystemTime::now();
            return true;
        }
        false
    }
}

pub(crate) trait TryElapsed<T> {
    fn try_elapsed(&self, duration: T) -> bool
    where
        Self: Sized;
}

impl TryElapsed<u64> for SystemTime {
    #[inline(always)]
    fn try_elapsed(&self, duration_ms: u64) -> bool
    where
        Self: Sized,
    {
        match self.elapsed() {
            Ok(time) => time >= Duration::from_millis(duration_ms),
            Err(_) => false,
        }
    }
}

impl TryElapsed<Duration> for SystemTime {
    #[inline(always)]
    fn try_elapsed(&self, duration: Duration) -> bool
    where
        Self: Sized,
    {
        match self.elapsed() {
            Ok(time) => time >= duration,
            Err(_) => false,
        }
    }
}