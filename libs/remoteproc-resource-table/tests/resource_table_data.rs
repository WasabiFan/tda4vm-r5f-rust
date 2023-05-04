use remoteproc_resource_table::{
    carveout::CarveoutResourceTypeData,
    packing::{ResourceTableTargetAddress, ZeroBytes},
    resource_table,
    trace::TraceResourceTypeData,
    FwResourceType,
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
    resource_table! {
        static test_log: TraceResourceTypeData = TraceResourceTypeData {
            device_address: ResourceTableTargetAddress(0x12345678 as *const u8),
            length: 100,
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
    resource_table! {
        static test_log_1: TraceResourceTypeData = TraceResourceTypeData {
            device_address: ResourceTableTargetAddress(0x12345678 as *const u8),
            length: 100,
            _reserved: ZeroBytes::new(),
            name: NAME_1,
        };
        static test_log_2: TraceResourceTypeData = TraceResourceTypeData {
            device_address: ResourceTableTargetAddress(0xabcdefab as *const u8),
            length: 200,
            _reserved: ZeroBytes::new(),
            name: NAME_2,
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

#[test]
fn test_three_entries_different_sizes() {
    #[repr(transparent)]
    pub struct Foo<const N: usize>([u8; N]);
    impl<const N: usize> Foo<N> {
        pub const fn get_resource_type() -> FwResourceType {
            FwResourceType::Trace
        }
    }

    // Given
    resource_table! {
        static test_entry_1: Foo<3> = Foo([1, 2, 3]);
        static test_entry_2: Foo<2> = Foo([4, 5]);
        static test_entry_3: Foo<1> = Foo([6]);
    };
    let actual = resource_table_bytes(&__REMOTEPROC_RESOURCE_TABLE);

    // Then
    let expected = concat_bytes![
        // Table header
        concat_bytes![
            // version
            1u32.to_le_bytes(),
            // num entries
            3u32.to_le_bytes(),
            // reserved
            [0u8; 8],
        ],
        // entry offsets
        concat_bytes![
            (16u32 + 12).to_le_bytes(),
            (16u32 + 12 + 7).to_le_bytes(),
            (16u32 + 12 + 7 + 6).to_le_bytes()
        ],
        // First entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [1, 2, 3],
        ],
        // Second entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [4, 5],
        ],
        // Third entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [6],
        ],
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_trace_from_buffer() {
    // Given
    static DUMMY_BUFFER: [u8; 100] = [0; 100];
    resource_table! {
        static test_log: TraceResourceTypeData = TraceResourceTypeData::from_buffer("123", &DUMMY_BUFFER);
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
        static test_vdev: CarveoutResourceTypeData = CarveoutResourceTypeData {
            device_address: ResourceTableTargetAddress(0x12345678 as *const u8),
            physical_address: ResourceTableTargetAddress(0xABCDEFAB as *const u8),
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
            // // type
            // 2u32.to_le_bytes(),
            // // da (device address)
            // // TODO: pointer size depends on host architecture when running tests
            // (0x12345678 as *const u8 as usize).to_le_bytes(),
            // // len
            // 100u32.to_le_bytes(),
            // // reserved
            // 0u32.to_le_bytes(),
            // // name
            // NAME,
        ]
    ];
    assert_eq!(actual, expected);
}
