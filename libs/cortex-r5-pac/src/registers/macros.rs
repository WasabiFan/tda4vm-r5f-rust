macro_rules! impl_readable_for_coprocessor_register {
    ($regtype:ty: $bitfield_type:ty, $cpnum:tt, $opc1:tt, $crn:tt, $crm:tt, $opc2:tt) => {
        impl tock_registers::interfaces::Readable for $regtype {
            type T = u32;
            type R = $bitfield_type;

            #[cfg(target_arch = "arm")]
            #[inline]
            fn get(&self) -> Self::T {
                let value;
                unsafe {
                    core::arch::asm!(
                        concat!(
                            "mrc ", stringify!($cpnum),
                            ", #", $opc1,
                            ", ", "{reg}",
                            ", ", stringify!($crn),
                            ", ", stringify!($crm),
                            ", #", $opc2,
                        ),
                        reg = out(reg) value,
                        options(nomem, nostack),
                    );
                };
                value
            }

            #[cfg(not(target_arch = "arm"))]
            fn get(&self) -> Self::T {
                unimplemented!();
            }
        }
    }
}

// TODO: Theoretically, only need barriers around modifications that affect specific bits in the register.
// Consider ways to avoid unnecessary barriers for other bits.

macro_rules! impl_writeable_for_coprocessor_register {
    (volatile $regtype:ty: $bitfield_type:ty, $cpnum:tt, $opc1:tt, $crn:tt, $crm:tt, $opc2:tt) => {
        impl_writeable_for_coprocessor_register!(
            $regtype: $bitfield_type, $cpnum, $opc1, $crn, $crm, $opc2,
            // Serialize all in-flight data transactions before we potentially modify memory mappings
            "dsb",
            // Ensure all future instruction fetches use new memory map.
            // This probably assumes that this instruction is in the same MPU region/page as the reg write.
            "isb",
            // Since this config change might affect arbitrary memory mappings, can't provide nomem/nostack
            options()
        );
    };
    ($regtype:ty: $bitfield_type:ty, $cpnum:tt, $opc1:tt, $crn:tt, $crm:tt, $opc2:tt) => {
        impl_writeable_for_coprocessor_register!(
            $regtype: $bitfield_type, $cpnum, $opc1, $crn, $crm, $opc2,
            "",
            "",
            options(nomem, nostack)
        );
    };
    ($regtype:ty: $bitfield_type:ty, $cpnum:tt, $opc1:tt, $crn:tt, $crm:tt, $opc2:tt, $pre_barriers:literal, $post_barriers:literal, options($($asm_opts:tt),*)) => {
        impl tock_registers::interfaces::Writeable for $regtype {
            type T = u32;
            type R = $bitfield_type;

            #[cfg(target_arch = "arm")]
            #[inline]
            fn set(&self, value: Self::T) {
                unsafe {
                    core::arch::asm!($pre_barriers);
                    core::arch::asm!(
                        concat!(
                            "mcr ", stringify!($cpnum),
                            ", #", $opc1,
                            ", ", "{reg}",
                            ", ", stringify!($crn),
                            ", ", stringify!($crm),
                            ", #", $opc2,
                        ),
                        reg = in(reg) value,
                        options($($asm_opts),*),
                    );
                    core::arch::asm!($post_barriers);
                };
            }

            #[cfg(not(target_arch = "arm"))]
            fn set(&self, _value: Self::T) {
                unimplemented!();
            }
        }
    }
}
