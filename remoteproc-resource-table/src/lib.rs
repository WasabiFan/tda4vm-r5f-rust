#![no_std]

#[repr(transparent)]
pub struct ResourceTableTargetAddress(pub *const u8);
unsafe impl Sync for ResourceTableTargetAddress {}

#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { $crate::count_tts!($($a)*) << 1 };
}

// TODO: use provided entry names to expose references to each entry

#[macro_export]
macro_rules! resource_table {
    [ $(static $entry_name:ident : $entry_type:ty = $entry_value:expr);*$(;)? ] => {
        const __REMOTEPROC_RESOURCE_TABLE_N: usize = $crate::count_tts!($($entry_name) *);

        #[cfg_attr(not(test), link_section = ".resource_table")]
        #[cfg_attr(not(test), no_mangle)]
        #[used]
        pub static __REMOTEPROC_RESOURCE_TABLE: ($crate::ResourceTableHeader<__REMOTEPROC_RESOURCE_TABLE_N>, ($($crate::ResourceEntry<$entry_type>),*)) = {
            const fn header<const N: usize>(sizes: [u32; N]) -> $crate::ResourceTableHeader<N> {
                let mut offset = [ 0u32; N ];

                if N > 0 {
                    offset[0] = core::mem::size_of::<$crate::ResourceTableHeader<N>>() as u32;

                    let mut i = 1;
                    loop {
                        if i >= N { break }
                        offset[i] = offset[i-1] + sizes[i-1];
                        i += 1;
                    }
                }

                $crate::ResourceTableHeader {
                    ver: 1,
                    num: N as u32,
                    _reserved: $crate::ZeroBytes::new(),
                    offset
                }
            }

            let sizes = [ $(core::mem::size_of::<$crate::ResourceEntry<$entry_type>>() as u32),*];
            let bodies = ( $(
                $crate::ResourceEntry {
                    resource_type: <$entry_type>::get_resource_type(),
                    data: $entry_value
                }
            ),* );

            (header(sizes), bodies)
        };
    }
}

pub struct ZeroBytes<const N: usize>([u8; N]);

impl<const N: usize> ZeroBytes<N> {
    pub const fn new() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Default for ZeroBytes<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub const fn fixed_length_str<const L: usize>(val: &str) -> [u8; L] {
    assert!(val.len() <= L);
    let mut result = [0; L];

    let mut i = 0;
    loop {
        if i >= val.len() {
            break;
        }
        result[i] = val.as_bytes()[i];
        i += 1;
    }

    result
}

#[repr(C, packed)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub _reserved: ZeroBytes<8>,
    pub offset: [u32; N],
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
    pub _reserved: ZeroBytes<4>,
    pub name: [u8; 32],
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
    pub _reserved: ZeroBytes<2>,
    pub vring: [VdevResourceVringDescriptor; N],
}

#[repr(C, packed)]
pub struct ResourceEntry<T> {
    pub resource_type: FwResourceType,
    pub data: T,
}
