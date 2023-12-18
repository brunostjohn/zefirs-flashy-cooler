use crate::Hardware;

pub struct HardwareIter<'a> {
    pub(crate) inner: &'a Hardware<'a>,
    pub(crate) index: usize,
    pub(crate) len: usize,
}

impl<'a> Iterator for HardwareIter<'a> {
    type Item = Hardware<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let hardware = Hardware {
                guard: self.inner.guard,
                indices: {
                    let mut indices = self.inner.indices.clone();

                    indices.push(self.index as i32);

                    indices
                },
            };

            self.index += 1;

            Some(hardware)
        } else {
            None
        }
    }
}
