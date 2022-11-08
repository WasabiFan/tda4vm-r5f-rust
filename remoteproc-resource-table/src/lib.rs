#![no_std]

#[repr(transparent)]
pub struct ResourceTableTargetAddress(pub *const u8);
unsafe impl Sync for ResourceTableTargetAddress {}

#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
}

#[macro_export]
macro_rules! resource_table {
    [ $($name:ident $body:tt),* ] => {
        const __REMOTEPROC_RESOURCE_TABLE_N: usize = $crate::count_tts!($($name) *);

        #[link_section = ".resource_table"]
        #[no_mangle]
        #[used]
        pub static __REMOTEPROC_RESOURCE_TABLE: ($crate::ResourceTableHeader<__REMOTEPROC_RESOURCE_TABLE_N>, ($($crate::ResourceEntry<$name>),*)) = {
            const fn header<const N: usize>(sizes: [u32; N]) -> $crate::ResourceTableHeader<N> {
                let mut offset = [ 0u32; N ];
                offset[0] = core::mem::size_of::<$crate::ResourceTableHeader<N>>() as u32;

                let mut i = 1;
                loop {
                    if i == N { break }
                    offset[i] = offset[i-1] + sizes[i-1];
                    i += 1;
                }

                $crate::ResourceTableHeader {
                    ver: 1,
                    num: N as u32,
                    _reserved: [0; 2],
                    offset
                }
            }

            let sizes = [ $(core::mem::size_of::<$name>() as u32),*];
            let bodies = ( $(
                $crate::ResourceEntry {
                    resource_type: $name::get_resource_type(),
                    data: $name $body
                }
            ),* );

            (header(sizes), bodies)
        };
    }
}

#[repr(C, packed)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub _reserved: [u32; 2],
    pub offset: [ u32; N ],
}

#[repr(u32)]
pub enum FwResourceType {
    Carveout = 0,
    DevMem = 1,
    Trace = 2,
    VDev = 3,
}

#[repr(C, packed)]
pub struct TraceResourceTypeData {
    pub device_address: ResourceTableTargetAddress,
    pub length: u32,
    pub _reserved: u32,
    pub name: [ u8; 32 ],
}

// TODO: implement resource types for other resources
impl TraceResourceTypeData {
    pub const fn get_resource_type() -> FwResourceType {
        FwResourceType::Trace
    }
}

#[repr(C, packed)]
pub struct VdevResourceVringDescriptor {
    pub device_address: u32,
    pub align: u32,
    pub num: u32,
    pub notifyid: u32,
    pub physical_address: u32,
}

#[repr(C, packed)]
pub struct VdevResourceTypeData<const N: usize> {
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u32,
    pub num_of_vrings: u8,
    // TODO: clean up "reserved" initialization
    pub reserved: [ u8; 2],
    pub vring: [VdevResourceVringDescriptor; N],
}

#[repr(C, packed)]
pub struct ResourceEntry<T> {
    pub resource_type: FwResourceType,
    pub data: T,
}
