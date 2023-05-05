use remoteproc_resource_table::{resource_table, FwResourceType};

mod utils;
use utils::resource_table_bytes;

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
