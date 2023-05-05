#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ResourceTableTargetAddress(*const u8);
unsafe impl Sync for ResourceTableTargetAddress {}

impl ResourceTableTargetAddress {
    /// Target addresss which indicates that the host kernel should select and fill in an address
    pub const ADDR_ANY: Self = Self(u32::MAX as *const u8);

    pub const fn new(address: *const u8) -> Self {
        Self(address)
    }

    /// Returns the contained pointer.
    /// 
    /// If the contained pointer is ResourceTableTargetAddress::ADDR_ANY, it is considered
    /// _uninitialized_ (i.e., the kernel did not populate the field when loading), and None is
    /// returned.
    pub fn read(&self) -> Option<*const u8> {
        if *self == ResourceTableTargetAddress::ADDR_ANY {
            None
        } else {
            Some(self.0)
        }
    }
}
