use crate::packing::ZeroBytes;

#[repr(C, packed)]
pub struct VdevResourceVringDescriptor {
    pub device_address: u32,
    pub align: u32,
    pub num: u32,
    pub notifyid: u32,
    pub physical_address: u32,
}

#[repr(C, packed)]
pub struct VdevResourceTypeData<const N: usize> {
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u32,
    pub num_of_vrings: u8,
    pub _reserved: ZeroBytes<2>,
    pub vring: [VdevResourceVringDescriptor; N],
}
