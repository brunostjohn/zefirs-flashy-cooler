use std::{
    sync::mpsc::Receiver,
    time::{Duration, SystemTime},
};

use super::traits::TryElapsed;

pub fn receive_flag(channel: &Receiver<bool>, assume: bool) -> bool {
    match channel.try_recv() {
        Ok(result) => return result,
        Err(_) => return assume,
    }
}

pub struct EventTicker {
    frequency: Duration,
    time: SystemTime,
}

impl EventTicker {
    pub fn new(frequency_ms: u64) -> Self {
        EventTicker {
            frequency: Duration::from_millis(frequency_ms),
            time: SystemTime::now(),
        }
    }

    pub fn check_time(&mut self) -> bool {
        if self.time.try_elapsed(self.frequency) {
            self.time = SystemTime::now();
            return true;
        }
        false
    }
}

pub trait ChangeFrequency<T> {
    fn change_frequency(&mut self, frequency: T);
}

impl ChangeFrequency<u64> for EventTicker {
    fn change_frequency(&mut self, frequency: u64) {
        self.frequency = Duration::from_millis(frequency);
    }
}

impl ChangeFrequency<&Receiver<Duration>> for EventTicker {
    fn change_frequency(&mut self, frequency: &Receiver<Duration>) {
        match frequency.try_recv() {
            Ok(freq) => self.frequency = freq,
            Err(_) => {}
        }
    }
}
