use tock_registers::register_bitfields;

register_bitfields![
    u32,

    /// CP15, MPU (Data) Region Access Control Registers
    pub DRACR [
        /// Execute-never (disallow execution), if IRACR is not supported
        XN OFFSET(12) NUMBITS(1) [
            ExecutionEnabled = 0,
            ExecutionDisabled = 1,
        ],

        /// Access Permissions
        AP OFFSET(8) NUMBITS(3) [
            AllNoAccess = 0b000,
            SupervisorReadWriteUserNoAccess = 0b001,
            SupervisorReadWriteUserReadOnly = 0b010,
            AllReadWrite = 0b011,
            // 0b100 is reserved
            SupervisorReadOnlyUserNoAccess = 0b101,
            AllReadOnly = 0b100,
            // 0b111 is reserved
        ],

        /// "Type Extension" originally, but now only meaningful in conjunction with C and B
        TEX OFFSET(3) NUMBITS(3) [],

        /// Shareable (i.e., coherent). Applicable only to "Normal" memory region types; ignored otherwise.
        S OFFSET(2) NUMBITS(1) [],

        /// "Cacheable" originally, but now only meaningful in conjunction with TEX and B
        C OFFSET(1) NUMBITS(1) [],

        /// "Bufferable" originally, but now only meaningful in conjunction with TEX and C
        B OFFSET(0) NUMBITS(1) [],
    ]
];

pub struct Reg;

impl_readable_for_coprocessor_register!(Reg: DRACR::Register, p15, 0, c6, c1, 4);
impl_writeable_for_coprocessor_register!(Reg: DRACR::Register, p15, 0, c6, c1, 4);

pub const DRACR: Reg = Reg;
