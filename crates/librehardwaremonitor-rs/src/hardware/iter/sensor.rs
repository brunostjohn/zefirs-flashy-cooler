use crate::{Hardware, Sensor};

pub struct SensorIter<'a> {
    pub(crate) inner: &'a Hardware<'a>,
    pub(crate) index: usize,
    pub(crate) len: usize,
}

impl<'a> Iterator for SensorIter<'a> {
    type Item = Sensor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let hardware = Sensor {
                computer_guard: self.inner.guard,
                hardware_guard: self.inner,
                index: self.index as i32,
            };

            self.index += 1;

            Some(hardware)
        } else {
            None
        }
    }
}
