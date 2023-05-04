use crate::{
    packing::{fixed_length_str, ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

#[repr(C, packed)]
pub struct CarveoutResourceTypeData {
    pub device_address: ResourceTableTargetAddress,
    pub physical_address: ResourceTableTargetAddress,
    pub length: u32,
    pub flags: u32,
    pub _reserved: ZeroBytes<4>,
    pub name: [u8; 32],
}

impl CarveoutResourceTypeData {
    pub const fn get_resource_type() -> FwResourceType {
        FwResourceType::Carveout
    }

    pub const fn new(name: &str, address: ResourceTableTargetAddress, length: u32) -> Self {
        Self {
            device_address: address,
            physical_address: ResourceTableTargetAddress::ADDR_ANY, // To be populated by host kernel
            length,
            flags: 0, // TODO
            _reserved: ZeroBytes::new(),
            name: fixed_length_str(name),
        }
    }
}
