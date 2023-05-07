use crate::{
    packing::{fixed_length_str, ResourceTableTargetAddress, ZeroBytes},
    FwResourceType,
};

#[repr(C)]
pub struct CarveoutResourceTypeData {
    device_address: ResourceTableTargetAddress,
    physical_address: ResourceTableTargetAddress,
    length: u32,
    flags: u32,
    _reserved: ZeroBytes<4>,
    name: [u8; 32],
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

#[cfg(test)]
mod tests {
    use crate::{
        carveout::CarveoutResourceTypeData,
        packing::{ResourceTableTargetAddress, ZeroBytes},
        test_utils::resource_table_bytes,
    };

    #[test]
    fn test_single_carveout_entry() {
        // Given
        const NAME: [u8; 32] = {
            let mut val = [0; 32];
            val[0] = 123;
            val[1] = 124;
            val[2] = 125;
            val[31] = 126;
            val
        };
        let vdev = CarveoutResourceTypeData {
            device_address: ResourceTableTargetAddress::with_value(0x12345678),
            physical_address: ResourceTableTargetAddress::with_value(0xABCDEFAB),
            length: 0x11223344,
            flags: 0x87654321,
            _reserved: ZeroBytes::new(),
            name: NAME,
        };
        let actual = resource_table_bytes(&vdev);

        // Then
        let expected = crate::concat_bytes![
            // da
            0x12345678u32.to_le_bytes(),
            // pa
            0xABCDEFABu32.to_le_bytes(),
            // len
            0x11223344u32.to_le_bytes(),
            // flags
            0x87654321u32.to_le_bytes(),
            // reserved
            0x00000000u32.to_le_bytes(),
            // name
            NAME,
        ];
        assert_eq!(actual, expected);
    }
}
