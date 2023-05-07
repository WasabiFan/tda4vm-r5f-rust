mod virtio_types;

use core::ptr::{addr_of, read_volatile};

use bitflags::bitflags;

use crate::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

pub use self::virtio_types::VirtIODeviceId;

#[repr(C)]
pub struct VdevResourceVringDescriptor {
    pub device_address: ResourceTableTargetAddress,
    pub align: u32,
    pub num: u32,
    pub notifyid: u32,
    pub physical_address: ResourceTableTargetAddress,
}

impl VdevResourceVringDescriptor {
    #[cfg(target_pointer_width = "32")]
    pub fn read_device_address(&self) -> Option<*const u8> {
        self.device_address.read()
    }

    #[cfg(target_pointer_width = "32")]
    pub fn read_physical_address(&self) -> Option<*const u8> {
        self.physical_address.read()
    }
}

// TODO: make most fields private
#[repr(C)]
pub struct VdevResourceTypeData<const N: usize> {
    pub id: VirtIODeviceId,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: VirtIOStatus,
    pub num_of_vrings: u8,
    pub _reserved: ZeroBytes<2>,
    pub vring: [VdevResourceVringDescriptor; N],
    // TODO: config space
}

impl<const N: usize> VdevResourceTypeData<N> {
    pub const fn get_resource_type() -> FwResourceType {
        FwResourceType::VDev
    }

    /// Reads and returns the current value of this resource's status field.
    ///
    /// Since the status field can be written by the higher powers (the host kernel) without
    /// synchronization, we enforce going through a blessed reading procedure rather than
    /// exposing it freely.
    pub fn read_status(&self) -> VirtIOStatus {
        let status_ptr: *const VirtIOStatus = addr_of!(self.status);
        unsafe { read_volatile(status_ptr) }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RpmsgFeatures: u32 {
        /// RP supports name service notifications
        const VIRTIO_RPMSG_F_NS = 1u32 << 0;
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct VirtIOStatus: u8 {
        /// Indicates that the guest OS has found the device and recognized it as a valid virtio device.
        const ACKNOWLEDGE = 1;
        /// Indicates that the guest OS knows how to drive the device. Note: There could be a significant (or infinite) delay before setting this bit. For example, under Linux, drivers can be loadable modules.
        const DRIVER = 2;
        /// Indicates that something went wrong in the guest, and it has given up on the device. This could be an internal error, or the driver didn’t like the device for some reason, or even a fatal error during device operation.
        const FAILED = 128;
        /// Indicates that the driver has acknowledged all the features it understands, and feature negotiation is complete.
        const FEATURES_OK = 8;
        /// Indicates that the driver is set up and ready to drive the device.
        const DRIVER_OK = 4;
        /// Indicates that the device has experienced an error from which it can’t recover.
        const DEVICE_NEEDS_RESET = 64;
    }
}
