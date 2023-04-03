use tock_registers::register_bitfields;

register_bitfields![
    u32,

    /// CP15, MPU (Data) Region Size and Enable Registers
    pub DRSR [
        // [16:31] Reserved

        /// Subregion disable (bit flags)
        SD OFFSET(8) NUMBITS(8) [],

        // [6:7] Reserved

        /// Region size (exponent)
        RSize OFFSET(1) NUMBITS(5) [],

        /// Enable
        En OFFSET(0) NUMBITS(1) [],
    ]
];

pub struct Reg;

impl_readable_for_coprocessor_register!(Reg: DRSR::Register, p15, 0, c6, c1, 2);
impl_writeable_for_coprocessor_register!(Reg: DRSR::Register, p15, 0, c6, c1, 2);

pub const DRSR: Reg = Reg;
