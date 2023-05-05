use remoteproc_resource_table::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    resource_table,
    trace::TraceResourceTypeData,
};

pub mod utils;
use utils::DummyTraceResourceData;

#[test]
fn test_arbitrary_expression_in_entry_decl() {
    // Given
    struct Foo;
    impl Foo {
        pub const fn make_a_trace() -> TraceResourceTypeData {
            TraceResourceTypeData {
                device_address: ResourceTableTargetAddress(0 as *const u8),
                length: 0,
                _reserved: ZeroBytes::new(),
                name: [0; 32],
            }
        }
    }
    resource_table![
        static FOO: TraceResourceTypeData = Foo::make_a_trace();
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
