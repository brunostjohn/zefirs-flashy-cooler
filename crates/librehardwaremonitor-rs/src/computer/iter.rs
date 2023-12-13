use crate::{Computer, Hardware};

pub struct ComputerHardwareIter<'a> {
    pub(crate) inner: &'a Computer,
    pub(crate) index: usize,
    pub(crate) len: usize,
}

impl<'a> Iterator for ComputerHardwareIter<'a> {
    type Item = Hardware<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let hardware = Hardware {
                guard: self.inner,
                indices: vec![self.index as i32],
            };

            self.index += 1;

            Some(hardware)
        } else {
            None
        }
    }
}
