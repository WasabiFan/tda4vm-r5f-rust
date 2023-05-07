use remoteproc_resource_table::{
    carveout::CarveoutResourceTypeData, packing::ResourceTableTargetAddress, resource_table,
};

pub mod utils;
use utils::DummyTraceResourceData;

#[test]
fn test_arbitrary_expression_in_entry_decl() {
    // Given
    resource_table![
        static FOO: CarveoutResourceTypeData = {
            let _x = 0;
            CarveoutResourceTypeData::new("foo", ResourceTableTargetAddress::with_value(0), 2)
        };
    ];

    // Then
    assert_eq!(__REMOTEPROC_RESOURCE_TABLE.__header.num as u32, 1);
}

#[test]
fn test_named_fields() {
    // Given

    resource_table![
        static FOO: DummyTraceResourceData<1> = DummyTraceResourceData([0]);
        static BAR: DummyTraceResourceData<1> = DummyTraceResourceData([1]);
    ];

    // Then
    assert_eq!(__REMOTEPROC_RESOURCE_TABLE.__header.num as u32, 2);
    assert_eq!(*FOO, DummyTraceResourceData([0]));
    assert_eq!(*BAR, DummyTraceResourceData([1]));
}
