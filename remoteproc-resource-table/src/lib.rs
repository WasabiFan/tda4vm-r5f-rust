#![no_std]

#[repr(transparent)]
pub struct ResourceTableTargetAddress(pub *const u8);
unsafe impl Sync for ResourceTableTargetAddress {}

#[repr(C, align(4096))]
pub struct ResourceTable<const N: usize> {
    pub header: ResourceTableHeader<N>,
    // TODO: I tried to be elegant, but this is broken without variadic generics. Need to figure
    // out a better structure.
    pub resources: [ResourceEntry<TraceResourceTypeData>; N],
}

#[repr(C, packed)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub _reserved: [u32; 2],
    pub offset: [ u32; N ],
}

#[repr(u32)]
pub enum FwResourceType {
    RSC_CARVEOUT = 0,
    RSC_DEVMEM = 1,
    RSC_TRACE = 2,
    RSC_VDEV = 3,
}

#[repr(C, packed)]
pub struct TraceResourceTypeData {
    pub device_address: ResourceTableTargetAddress,
    pub length: u32,
    pub _reserved: u32,
    pub name: [ u8; 32 ],
}

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
    pub reserved: [ u8; 2],
    pub vring: [VdevResourceVringDescriptor; N],
}


#[repr(C, packed)]
pub struct ResourceEntry<T> {
    pub resource_type: FwResourceType,
    pub data: T,
}
