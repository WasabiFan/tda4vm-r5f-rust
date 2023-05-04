#[repr(transparent)]
pub struct ResourceTableTargetAddress(pub *const u8);
unsafe impl Sync for ResourceTableTargetAddress {}

impl ResourceTableTargetAddress {
    /// Target addresss which indicates that the host kernel should select and fill in an address
    pub const ADDR_ANY: Self = Self(u32::MAX as *const u8);
}
