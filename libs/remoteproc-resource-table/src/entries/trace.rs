use crate::{
    packing::{fixed_length_str, ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

#[repr(C, packed)]
pub struct TraceResourceTypeData {
    pub device_address: ResourceTableTargetAddress,
    pub length: u32,
    pub _reserved: ZeroBytes<4>,
    pub name: [u8; 32],
}

impl TraceResourceTypeData {
    pub const fn get_resource_type() -> FwResourceType {
        FwResourceType::Trace
    }

    pub const fn from_buffer(name: &str, buffer: &[u8]) -> Self {
        Self {
            device_address: ResourceTableTargetAddress::new(buffer.as_ptr()),
            length: buffer.len() as u32,
            _reserved: ZeroBytes::new(),
            name: fixed_length_str(name),
        }
    }
}
