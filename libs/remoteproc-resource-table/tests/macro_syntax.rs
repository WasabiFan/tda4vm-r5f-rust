use remoteproc_resource_table::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    resource_table,
    trace::TraceResourceTypeData,
};

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
        static foo: TraceResourceTypeData = Foo::make_a_trace();
    ];

    // Then
    assert_eq!(__REMOTEPROC_RESOURCE_TABLE.0.num as u32, 1);
}
