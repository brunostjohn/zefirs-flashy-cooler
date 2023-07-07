use std::{
    sync::mpsc::Receiver,
    time::{Duration, SystemTime},
};

pub trait Reassign<T> {
    fn reassign(self, channel: T) -> Self
    where
        Self: Sized;
}

impl Reassign<&Receiver<Duration>> for Duration {
    fn reassign(self, channel: &Receiver<Self>) -> Self
    where
        Self: Sized,
    {
        match channel.try_recv() {
            Ok(result) => return Self::from(result),
            Err(_) => return self,
        }
    }
}

pub trait TryElapsed<T> {
    fn try_elapsed(&self, duration: T) -> bool
    where
        Self: Sized;
}

impl TryElapsed<u64> for SystemTime {
    fn try_elapsed(&self, duration_ms: u64) -> bool
    where
        Self: Sized,
    {
        match self.elapsed() {
            Ok(time) => {
                if time >= Duration::from_millis(duration_ms) {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }
}

impl TryElapsed<Duration> for SystemTime {
    fn try_elapsed(&self, duration: Duration) -> bool
    where
        Self: Sized,
    {
        match self.elapsed() {
            Ok(time) => {
                if time >= duration {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }
}
