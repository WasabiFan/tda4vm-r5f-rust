/// Simple trace buffer logic which wraps to the start of the buffer when full. Does not specially
/// handle newlines or ensure each message ends up contiguous.
pub struct CircularTraceBuffer<const LENGTH: usize> {
    pub buffer: [u8; LENGTH],
    next_byte_idx: usize,
}

impl<const LENGTH: usize> CircularTraceBuffer<LENGTH> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; LENGTH],
            next_byte_idx: 0,
        }
    }

    /// Highly suspect hack to allow retrieval of length
    pub const fn length(_r: &Self) -> usize {
        LENGTH
    }
}

impl<const LENGTH: usize> core::fmt::Write for CircularTraceBuffer<LENGTH> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        assert!(LENGTH >= 2);

        for c in s.as_bytes().iter() {
            self.buffer[self.next_byte_idx] = *c;
            // Preserve final byte as null terminator
            self.next_byte_idx = (self.next_byte_idx + 1) % (LENGTH - 1);
        }

        core::fmt::Result::Ok(())
    }
}
