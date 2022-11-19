use remoteproc_resource_table::{
    resource_table, ResourceTableTargetAddress, TraceResourceTypeData,
};

fn resource_table_bytes<'a, T>(resource_table: &'a T) -> &'a [u8] {
    let start = resource_table as *const _ as *const u8;
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(start, size) }
}

macro_rules! concat_bytes {
    [$($val:expr),* $(,)?] => { [].into_iter()$(.chain($val))*.collect::<Vec<u8>>() }
}

#[test]
fn test_empty_resource_table() {
    // Given
    resource_table![];
    let actual = resource_table_bytes(&__REMOTEPROC_RESOURCE_TABLE);

    // Then
    let expected = concat_bytes![
        // version
        1u32.to_le_bytes(),
        // num entries
        0u32.to_le_bytes(),
        // reserved
        [0u8; 8],
    ];
    assert_eq!(actual, expected);
}

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
    resource_table![TraceResourceTypeData {
        device_address: ResourceTableTargetAddress(0x12345678 as *const u8),
        length: 100,
        _reserved: 0,
        name: NAME,
    }];
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
            2u32.to_le_bytes(),
            // da (device address)
            // TODO: pointer size depends on host architecture when running tests
            (0x12345678 as *const u8 as usize).to_le_bytes(),
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

#[test]
fn test_two_trace_entries() {
    // Given
    const NAME_1: [u8; 32] = {
        let mut val = [0; 32];
        val[0] = 123;
        val[1] = 124;
        val[2] = 125;
        val[31] = 126;
        val
    };
    const NAME_2: [u8; 32] = {
        let mut val = [0; 32];
        val[0] = 234;
        val[1] = 235;
        val[2] = 236;
        val[31] = 237;
        val
    };
    resource_table![
        TraceResourceTypeData {
            device_address: ResourceTableTargetAddress(0x12345678 as *const u8),
            length: 100,
            _reserved: 0,
            name: NAME_1,
        },
        TraceResourceTypeData {
            device_address: ResourceTableTargetAddress(0xabcdefab as *const u8),
            length: 200,
            _reserved: 0,
            name: NAME_2,
        }
    ];
    let actual = resource_table_bytes(&__REMOTEPROC_RESOURCE_TABLE);

    // Then
    let expected = concat_bytes![
        // Table header
        concat_bytes![
            // version
            1u32.to_le_bytes(),
            // num entries
            2u32.to_le_bytes(),
            // reserved
            [0u8; 8],
        ],
        // entry offsets
        concat_bytes![(16u32 + 8).to_le_bytes(), (16u32 + 8 + 52).to_le_bytes()],
        // First entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // da (device address)
            // TODO: pointer size depends on host architecture when running tests
            (0x12345678 as *const u8 as usize).to_le_bytes(),
            // len
            100u32.to_le_bytes(),
            // reserved
            0u32.to_le_bytes(),
            // name
            NAME_1,
        ],
        // Second entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // da (device address)
            // TODO: pointer size depends on host architecture when running tests
            (0xabcdefab as *const u8 as usize).to_le_bytes(),
            // len
            200u32.to_le_bytes(),
            // reserved
            0u32.to_le_bytes(),
            // name
            NAME_2,
        ],
    ];
    assert_eq!(actual, expected);
}
