use crate::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

#[repr(C)]
pub struct TraceResourceTypeData {
    device_address: ResourceTableTargetAddress,
    length: u32,
    _reserved: ZeroBytes<4>,
    name: [u8; 32],
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

#[cfg(test)]
mod tests {
    use crate::{
        packing::{ResourceTableTargetAddress, ZeroBytes},
        test_utils::resource_table_bytes,
        trace::TraceResourceTypeData,
    };

    #[test]
    fn test_single_trace_entry() {
        // Given
        const NAME: [u8; 32] = {
            let mut val = [0; 32];
            val[0] = 123;
            val[1] = 124;
            val[2] = 125;
            val[31] = 126;
            val
        };
        let trace = TraceResourceTypeData {
            device_address: ResourceTableTargetAddress::with_value(0x12345678),
            length: 100,
            _reserved: ZeroBytes::new(),
            name: NAME,
        };
        let actual = resource_table_bytes(&trace);

        // Then
        let expected = crate::concat_bytes![
            // da (device address)
            0x12345678u32.to_le_bytes(),
            // len
            100u32.to_le_bytes(),
            // reserved
            0u32.to_le_bytes(),
            // name
            NAME,
        ];
        assert_eq!(actual, expected);
    }
}
