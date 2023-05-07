use remoteproc_resource_table::{
    carveout::CarveoutResourceTypeData, packing::ResourceTableTargetAddress, resource_table,
};

pub mod utils;
use utils::resource_table_bytes;

#[test]
fn test_carveout_new() {
    // Given
    const NAME: [u8; 32] = {
        let mut val = [0; 32];
        val[0] = 0x31;
        val[1] = 0x32;
        val[2] = 0x33;
        val
    };
    resource_table! {
        static TEST_VDEV: CarveoutResourceTypeData = CarveoutResourceTypeData::new(
            "123",
            ResourceTableTargetAddress::with_value(0x12345678),
            0x11223344,
        );
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
            0x12345678u32.to_le_bytes(),
            // pa
            0xffffffffu32.to_le_bytes(),
            // len
            0x11223344u32.to_le_bytes(),
            // flags
            0x00000000u32.to_le_bytes(),
            // reserved
            0x00000000u32.to_le_bytes(),
            // name
            NAME,
        ]
    ];
    assert_eq!(actual, expected);
}
