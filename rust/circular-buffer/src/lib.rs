
#[derive(PartialEq, Debug)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

#[derive(PartialEq, Debug)]
pub struct CircularBuffer<T> {
    head: usize,
    len: usize,
    storage: Vec<T>,
}

impl<T> CircularBuffer<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> CircularBuffer<T> {
        CircularBuffer {
            head: 0,
            len: 0,
            storage: Vec::with_capacity(capacity),
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.storage.capacity()
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let value = self.storage[self.head].clone();
            self.forward_head();
            Ok(value)
        }
    }

    fn forward_head(&mut self) {
        self.head = (self.head + 1) % self.storage.capacity();
        self.len -= 1;
    }

    pub fn write(&mut self, value: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            let tail = (self.head + self.len) % self.storage.capacity();
            self.store(tail, value);
            self.len += 1;
            Ok(())
        }
    }

    fn store(&mut self, index: usize, value: T) {
        if self.storage.len() == self.storage.capacity() {
            self.storage[index] = value;
        } else {
            assert_eq!(index, self.storage.len());
            self.storage.push(value);
        }
    }

    pub fn overwrite(&mut self, value: T) {
        if self.is_full() {
            self.forward_head();
        }
        self.write(value).expect("value writen now space was made");
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}
