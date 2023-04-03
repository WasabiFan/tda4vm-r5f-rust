use tock_registers::register_bitfields;

register_bitfields![
    u32,

    /// MPU (Data) Region Base Address Register
    pub DRBAR [
        /// Region base address, aligned to region size (at least 4 bytes).
        BaseAddress OFFSET(0) NUMBITS(32),
    ]
];

pub struct Reg;

impl_readable_for_coprocessor_register!(Reg: DRBAR::Register, p15, 0, c6, c1, 0);
impl_writeable_for_coprocessor_register!(Reg: DRBAR::Register, p15, 0, c6, c1, 0);

pub const DRBAR: Reg = Reg;
