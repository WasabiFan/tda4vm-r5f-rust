use remoteproc_resource_table::resource_table;

pub mod utils;
use utils::resource_table_bytes;

#[test]
#[ignore = "from_buffer not available on host architecture"]
fn test_trace_from_buffer() {
    // Given
    static DUMMY_BUFFER: [u8; 100] = [0; 100];
    resource_table! {
        // static TEST_LOG: TraceResourceTypeData = TraceResourceTypeData::from_buffer("123", &DUMMY_BUFFER);
    };
    let actual = resource_table_bytes(&__REMOTEPROC_RESOURCE_TABLE);

    // Then
    const NAME: [u8; 32] = {
        let mut val = [0; 32];
        val[0] = 0x31;
        val[1] = 0x32;
        val[2] = 0x33;
        val
    };

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
            2u32.to_le_bytes(),
            // da (device address)
            // TODO: pointer size depends on host architecture when running tests
            (DUMMY_BUFFER.as_ptr() as usize).to_le_bytes(),
            // len
            100u32.to_le_bytes(),
            // reserved
            0u32.to_le_bytes(),
            // name
            NAME,
        ]
    ];
    assert_eq!(actual, expected);
}
