//! Types to represent device-side pointers (32-bit) in resource tables.
//! 
//! This address type must store a pointer (not plain integer), because resource table entries are often initialized via
//! an an unresolved linker symbol. It has a 4-byte memory representation.
//! 
//! To enable testing of resource table generation, we need to retain the 4-byte representation on 64-bit systems. We've
//! shimmed the type on 64-bit targets to store a 32-bit integer, which is sufficient for faithful representation tests.

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg(target_pointer_width = "32")]
pub struct ResourceTableTargetAddress(*const u8);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg(not(target_pointer_width = "32"))]
pub struct ResourceTableTargetAddress(u32);

unsafe impl Sync for ResourceTableTargetAddress {}

impl ResourceTableTargetAddress {
    const ADDR_ANY_VAL: u32 = u32::MAX;
    /// Target addresss which indicates that the host kernel should select and fill in an address
    pub const ADDR_ANY: Self = Self::with_value(Self::ADDR_ANY_VAL);

    #[cfg(target_pointer_width = "32")]
    pub const fn from_pointer(p: *const u8) -> Self {
        Self(p)
    }

    #[cfg(target_pointer_width = "32")]
    pub const fn with_value(val: u32) -> Self {
        Self(val as *const u8)
    }

    #[cfg(not(target_pointer_width = "32"))]
    pub const fn with_value(val: u32) -> Self {
        Self(val)
    }

    /// Returns the contained pointer.
    ///
    /// If the contained pointer is ResourceTableTargetAddress::ADDR_ANY, it is considered
    /// _uninitialized_ (i.e., the kernel did not populate the field when loading), and None is
    /// returned.
    #[cfg(target_pointer_width = "32")]
    pub fn read(&self) -> Option<*const u8> {
        let self_addr: *const *const u8 = core::ptr::addr_of!(self.0);
        let val = unsafe { core::ptr::read_volatile(self_addr) };
        if val as u32 == Self::ADDR_ANY_VAL {
            None
        } else {
            Some(val)
        }
    }
}
