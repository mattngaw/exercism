pub struct CircularBuffer<T: Clone + Default> {
    data: Vec<T>,
    capacity: usize,
    length: usize,
    start: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let data = vec![Default::default(); capacity];
        Self {
            data,
            capacity, 
            length: 0,
            start: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.length == self.capacity { return Err(Error::FullBuffer) }
        let i = (self.start + self.length) % self.capacity;
        self.data[i] = element;
        self.length += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.length == 0 { return Err(Error::EmptyBuffer) }
        let value = self.data[self.start].clone();
        self.start = (self.start + 1) % self.capacity;
        self.length -= 1;
        Ok(value)
    }

    pub fn clear(&mut self) {
        for e in self.data.iter_mut() {
            *e = Default::default();
        }
        self.length = 0;
        self.start = 0;
        return
    }

    pub fn overwrite(&mut self, element: T) {
        if self.length < self.capacity {
            let _ = self.write(element);
            return
        }
        self.data[self.start] = element;
        self.start = (self.start + 1) % self.capacity;
        return
    }
}
