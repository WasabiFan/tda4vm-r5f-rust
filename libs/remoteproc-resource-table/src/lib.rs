#![no_std]

mod entries;
pub mod packing;

pub use entries::carveout;
pub use entries::trace;
pub use entries::vdev;

pub use entries::FwResourceType;

use packing::ZeroBytes;

pub extern crate memoffset;

#[macro_export]
macro_rules! count_tts {
    () => { 0 };
    ($odd:tt $($a:tt $b:tt)*) => { ($crate::count_tts!($($a)*) << 1) | 1 };
    ($($a:tt $even:tt)*) => { $crate::count_tts!($($a)*) << 1 };
}

#[macro_export]
macro_rules! resource_table {
    [ $(static $entry_name:ident : $entry_type:ty = $entry_value:expr);*$(;)? ] => {
        const __REMOTEPROC_RESOURCE_TABLE_N: usize = $crate::count_tts!($($entry_name) *);

        #[repr(C)]
        #[allow(non_snake_case)]
        pub struct __REMOTEPROC_RESOURCE_TABLE_STRUCT {
            __header: $crate::ResourceTableHeader<__REMOTEPROC_RESOURCE_TABLE_N>,
            $($entry_name: $crate::ResourceEntry<$entry_type>),*
        }

        #[cfg_attr(not(test), link_section = ".resource_table")]
        #[cfg_attr(not(test), no_mangle)]
        #[used]
        pub static __REMOTEPROC_RESOURCE_TABLE: __REMOTEPROC_RESOURCE_TABLE_STRUCT = {
            let offset: [ u32; __REMOTEPROC_RESOURCE_TABLE_N ] = [
                $($crate::memoffset::offset_of!(__REMOTEPROC_RESOURCE_TABLE_STRUCT, $entry_name) as u32),*
            ];

            let header = $crate::ResourceTableHeader {
                ver: 1,
                num: __REMOTEPROC_RESOURCE_TABLE_N as u32,
                _reserved: $crate::packing::ZeroBytes::new(),
                offset
            };

            __REMOTEPROC_RESOURCE_TABLE_STRUCT {
                __header: header,
                $(
                    $entry_name: $crate::ResourceEntry {
                        resource_type: <$entry_type>::get_resource_type(),
                        data: $entry_value
                    }
                ),*
            }
        };

        $(pub static $entry_name: &$entry_type = &__REMOTEPROC_RESOURCE_TABLE. $entry_name.data);*;
    }
}

#[repr(C)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub _reserved: ZeroBytes<8>,
    pub offset: [u32; N],
}

#[repr(C)]
pub struct ResourceEntry<T> {
    pub resource_type: FwResourceType,
    pub data: T,
}
