mod virtio_types;

use bitflags::bitflags;

use crate::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

pub use self::virtio_types::VirtIODeviceId;

#[repr(C, packed)]
pub struct VdevResourceVringDescriptor {
    pub device_address: ResourceTableTargetAddress,
    pub align: u32,
    pub num: u32,
    pub notifyid: u32,
    pub physical_address: ResourceTableTargetAddress,
}

#[repr(C, packed)]
pub struct VdevResourceTypeData<const N: usize> {
    pub id: VirtIODeviceId,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u8,
    pub num_of_vrings: u8,
    pub _reserved: ZeroBytes<2>,
    pub vring: [VdevResourceVringDescriptor; N],
    // TODO: config space
}

impl<const N: usize> VdevResourceTypeData<N> {
    pub const fn get_resource_type() -> FwResourceType {
        FwResourceType::VDev
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
