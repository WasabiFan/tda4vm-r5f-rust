use crate::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

#[repr(C)]
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

    #[cfg(target_pointer_width = "32")]
    pub const fn from_buffer(name: &str, buffer: &[u8]) -> Self {
        Self {
            device_address: ResourceTableTargetAddress::from_pointer(buffer.as_ptr()),
            length: buffer.len() as u32,
            _reserved: ZeroBytes::new(),
            name: crate::packing::fixed_length_str(name),
        }
    }
}
