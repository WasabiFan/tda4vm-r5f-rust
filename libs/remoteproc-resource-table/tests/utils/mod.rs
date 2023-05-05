use remoteproc_resource_table::FwResourceType;

pub fn resource_table_bytes<'a, T>(resource_table: &'a T) -> &'a [u8] {
    let start = resource_table as *const _ as *const u8;
    let size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(start, size) }
}

#[macro_export]
macro_rules! concat_bytes {
    [$($val:expr),* $(,)?] => { [].into_iter()$(.chain($val))*.collect::<Vec<u8>>() }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq)]
pub struct DummyTraceResourceData<const N: usize>(pub [u8; N]);
impl<const N: usize> DummyTraceResourceData<N> {
    pub const fn get_resource_type() -> FwResourceType {
        FwResourceType::Trace
    }
}
