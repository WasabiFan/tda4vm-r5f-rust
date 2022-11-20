struct ChunkedScrollingTraceBuffer<const LENGTH: usize> {
    pub buffer: [u8; LENGTH],
    used_length: usize,
}

impl<const LENGTH: usize> ChunkedScrollingTraceBuffer<LENGTH> {
    pub fn new() -> Self {
        Self {
            buffer: [0; LENGTH],
            used_length: 0,
        }
    }
}

impl<const LENGTH: usize> core::fmt::Write for ChunkedScrollingTraceBuffer<LENGTH> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let bytes = if bytes.len() > LENGTH {
            let surplus = bytes.len() - LENGTH;
            &bytes[surplus..]
        } else {
            bytes
        };

        let desired_used_length = self.used_length + bytes.len();
        if desired_used_length > LENGTH {
            // Reserve at least 1/3 of the buffer each time we shuffle
            const LENGTH_MINUS_MARGIN: usize = (LENGTH / 3) * 2;
            let existing_data_remaining_length = LENGTH_MINUS_MARGIN.saturating_sub(bytes.len());
            let existing_data_dropped_length = self.used_length - existing_data_remaining_length;
            unsafe {
                core::ptr::copy(self.buffer[existing_data_dropped_length..].as_mut_ptr(), self.buffer.as_mut_ptr(), existing_data_remaining_length);
            }
            self.buffer[]
            // self.buffer[..existing_data_new_length].copy_from_slice(src)
        }

        self.buffer[used_length..used_length+bytes.len()].copy_from_slice(bytes);
    }
}