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
        static TEST_ENTRY_1: DummyTraceResourceData<3> = DummyTraceResourceData([1, 2, 3]);
        static TEST_ENTRY_2: DummyTraceResourceData<2> = DummyTraceResourceData([4, 5]);
        static TEST_ENTRY_3: DummyTraceResourceData<1> = DummyTraceResourceData([6]);
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
