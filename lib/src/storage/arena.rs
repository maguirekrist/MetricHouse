

pub struct Arena {
    buffer: Vec<u8>,
    offset: usize
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            offset: 0,
        }
    }

    pub fn alloc_str<'a>(&'a mut self, input: &str) -> &'a str {
        let bytes = input.as_bytes();
        let len = bytes.len();

        // Ensure enough capacity
        if self.offset + len > self.buffer.capacity() {
            panic!("Arena out of memory");
        }

        // Extend buffer (doesn't reallocate because of capacity)
        self.buffer.resize(self.offset + len, 0);
        self.buffer[self.offset..self.offset + len].copy_from_slice(bytes);

        // Get the slice
        let ptr = &self.buffer[self.offset] as *const u8;
        self.offset += len;

        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
        }
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }
}