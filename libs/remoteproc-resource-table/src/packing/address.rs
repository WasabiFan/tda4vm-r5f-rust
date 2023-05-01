#[repr(transparent)]
pub struct ResourceTableTargetAddress(pub *const u8);
unsafe impl Sync for ResourceTableTargetAddress {}
