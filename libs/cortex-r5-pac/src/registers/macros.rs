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

macro_rules! impl_writeable_for_coprocessor_register {
    ($regtype:ty: $bitfield_type:ty, $cpnum:tt, $opc1:tt, $crn:tt, $crm:tt, $opc2:tt) => {
        impl tock_registers::interfaces::Writeable for $regtype {
            type T = u32;
            type R = $bitfield_type;

            #[cfg(target_arch = "arm")]
            #[inline]
            fn set(&self, value: Self::T) {
                unsafe {
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
                        options(nomem, nostack),
                    );
                };
            }

            #[cfg(not(target_arch = "arm"))]
            fn set(&self, _value: Self::T) {
                unimplemented!();
            }
        }
    }
}
