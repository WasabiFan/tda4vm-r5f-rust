use remoteproc_resource_table::{
    carveout::CarveoutResourceTypeData,
    packing::{ResourceTableTargetAddress, ZeroBytes},
    resource_table,
};

pub mod utils;
use utils::resource_table_bytes;

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
    resource_table! {
        static TEST_VDEV: CarveoutResourceTypeData = CarveoutResourceTypeData {
            device_address: ResourceTableTargetAddress::new(0x12345678 as *const u8),
            physical_address: ResourceTableTargetAddress::new(0xABCDEFAB as *const u8),
            length: 0x11223344,
            flags: 0x87654321,
            _reserved: ZeroBytes::new(),
            name: NAME,
        };
    };
    let actual = resource_table_bytes(&__REMOTEPROC_RESOURCE_TABLE);

    // Then
    let expected = concat_bytes![
        // Table header
        concat_bytes![
            // version
            1u32.to_le_bytes(),
            // num entries
            1u32.to_le_bytes(),
            // reserved
            [0u8; 8],
        ],
        // entry offsets
        concat_bytes![20u32.to_le_bytes()],
        // First entry
        concat_bytes![
            // type
            0u32.to_le_bytes(),
            // da
            (0x12345678u32 as *const u8 as usize).to_le_bytes(),
            // pa
            (0xABCDEFABu32 as *const u8 as usize).to_le_bytes(),
            // len
            0x11223344u32.to_le_bytes(),
            // flags
            0x87654321u32.to_le_bytes(),
            // reserved
            0x00000000u32.to_le_bytes(),
            // name
            NAME,
        ]
    ];
    assert_eq!(actual, expected);
}
