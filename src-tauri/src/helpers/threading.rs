use std::{
    sync::mpsc::Receiver,
    thread,
    time::{Duration, SystemTime},
};

#[path = "./traits.rs"]
mod helpers_traits;
use helpers_traits::TryElapsed;

#[inline(always)]
pub fn receive_flag(channel: &Receiver<bool>, assume: bool) -> bool {
    match channel.try_recv() {
        Ok(result) => return result,
        Err(_) => return assume,
    }
}

pub struct EventTicker {
    frequency: Duration,
    #[allow(dead_code)]
    time: SystemTime,
}

#[allow(dead_code)]
impl EventTicker {
    pub fn new(frequency_ms: u64) -> Self {
        EventTicker {
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

    #[inline(always)]
    pub fn wait_for_next(&mut self) {
        match self.time.elapsed() {
            Ok(dur) => {
                if dur < self.frequency {
                    thread::sleep(self.frequency - dur);
                }
            }
            Err(_) => {
                thread::sleep(self.frequency);
            }
        }

        self.time = SystemTime::now();
    }
}

pub trait ChangeFrequency<T> {
    fn change_frequency(&mut self, frequency: T);
}

impl ChangeFrequency<u64> for EventTicker {
    #[inline(always)]
    fn change_frequency(&mut self, frequency: u64) {
        self.frequency = Duration::from_millis(frequency);
    }
}

impl ChangeFrequency<&Receiver<Duration>> for EventTicker {
    #[inline(always)]
    fn change_frequency(&mut self, frequency: &Receiver<Duration>) {
        match frequency.try_recv() {
            Ok(freq) => self.frequency = freq,
            Err(_) => {}
        }
    }
}

impl ChangeFrequency<&Receiver<u64>> for EventTicker {
    #[inline(always)]
    fn change_frequency(&mut self, frequency: &Receiver<u64>) {
        match frequency.try_recv() {
            Ok(freq) => self.frequency = Duration::from_millis(freq),
            Err(_) => {}
        }
    }
}
