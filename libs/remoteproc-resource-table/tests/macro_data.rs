use remoteproc_resource_table::resource_table;

pub mod utils;
use utils::{resource_table_bytes, DummyTraceResourceData};

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
fn test_three_entries_different_sizes() {
    // Given
    resource_table! {
        static TEST_ENTRY_1: DummyTraceResourceData<12> = DummyTraceResourceData([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        static TEST_ENTRY_2: DummyTraceResourceData<8> = DummyTraceResourceData([13, 14, 15, 16, 17, 18, 19, 20]);
        static TEST_ENTRY_3: DummyTraceResourceData<4> = DummyTraceResourceData([21, 22, 23, 24]);
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
            (16u32 + 12 + 16).to_le_bytes(),
            (16u32 + 12 + 16 + 12).to_le_bytes()
        ],
        // First entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        ],
        // Second entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [13, 14, 15, 16, 17, 18, 19, 20],
        ],
        // Third entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [21, 22, 23, 24],
        ],
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_enforced_alignment() {
    // Given
    resource_table! {
        static TEST_ENTRY_1: DummyTraceResourceData<3> = DummyTraceResourceData([1, 2, 3]);
        static TEST_ENTRY_2: DummyTraceResourceData<1> = DummyTraceResourceData([99]);
    };
    let actual = resource_table_bytes(&__REMOTEPROC_RESOURCE_TABLE);

    // Then
    let expected_entry_1_length = 8;
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
        concat_bytes![
            (16u32 + 8).to_le_bytes(),
            (16u32 + 8 + expected_entry_1_length).to_le_bytes(),
        ],
        // First entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [1, 2, 3],
        ],
        // One byte of padding
        [0],
        // Second entry
        concat_bytes![
            // type
            2u32.to_le_bytes(),
            // data
            [99],
        ],
        // Three bytes of padding
        [0, 0, 0],
    ];
    assert_eq!(actual, expected);
}
