/// Opaque struct whose memory representation is an all-zeroes byte sequence of the specified
/// length. Intended to be used for "reserved" fields in packed structs.
pub struct ZeroBytes<const N: usize>([u8; N]);

impl<const N: usize> ZeroBytes<N> {
    pub const fn new() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Default for ZeroBytes<N> {
    fn default() -> Self {
        Self::new()
    }
}
