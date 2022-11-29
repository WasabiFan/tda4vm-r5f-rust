use tock_registers::register_bitfields;

register_bitfields![
    u32,

    /// CP15, RGNR, MPU Region Number Register
    pub RGNR [
        /// Region number
        Region OFFSET(0) NUMBITS(32) [],
    ]
];

pub struct Reg;

impl_readable_for_coprocessor_register!(Reg: RGNR::Register, p15, 0, c6, c2, 0);
impl_writeable_for_coprocessor_register!(Reg: RGNR::Register, p15, 0, c6, c2, 0);

pub const RGNR: Reg = Reg;
