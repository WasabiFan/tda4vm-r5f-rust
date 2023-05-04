#![no_std]

mod entries;
pub mod packing;

pub use entries::carveout;
pub use entries::trace;
pub use entries::vdev;

pub use entries::FwResourceType;

use packing::ZeroBytes;

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

        #[repr(C)]
        pub struct __REMOTEPROC_RESOURCE_TABLE_STRUCT(
            $crate::ResourceTableHeader<__REMOTEPROC_RESOURCE_TABLE_N>,
            $($crate::ResourceEntry<$entry_type>),*
        );

        #[cfg_attr(not(test), link_section = ".resource_table")]
        #[cfg_attr(not(test), no_mangle)]
        #[used]
        pub static __REMOTEPROC_RESOURCE_TABLE: __REMOTEPROC_RESOURCE_TABLE_STRUCT = {
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
                    _reserved: $crate::packing::ZeroBytes::new(),
                    offset
                }
            }

            let sizes = [ $(core::mem::size_of::<$crate::ResourceEntry<$entry_type>>() as u32),*];

            __REMOTEPROC_RESOURCE_TABLE_STRUCT(
                header(sizes),
                $(
                    $crate::ResourceEntry {
                        resource_type: <$entry_type>::get_resource_type(),
                        data: $entry_value
                    }
                ),*
            )
        };
    }
}

#[repr(C, packed)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub _reserved: ZeroBytes<8>,
    pub offset: [u32; N],
}

#[repr(C, packed)]
pub struct ResourceEntry<T> {
    pub resource_type: FwResourceType,
    pub data: T,
}
