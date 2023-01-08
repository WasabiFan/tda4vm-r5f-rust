use tock_registers::register_bitfields;

register_bitfields![
    u32,

    /// CP15, System Control Register
    pub SCTLR [
        /// Instruction endianness (read-only)
        IE OFFSET(31) NUMBITS(1) [
            LittleEndian = 0,
            BigEndian = 1,
        ],

        /// Thumb exception enable
        TE OFFSET(30) NUMBITS(1) [
            ArmExceptions = 0,
            ThumbExceptions = 1,
        ],

        /// Access Flag Enable (should be zero)
        AFE OFFSET(29) NUMBITS(1) [],

        /// TEX Remap Enable (should be zero)
        TRE OFFSET(28) NUMBITS(1) [],

        /// Non-maskable fast interrupt enable (read-only)
        NMFI OFFSET(27) NUMBITS(1) [],

        // [26] undefined, should be zero

        /// CPSR "E" bit value on exception
        EE OFFSET(25) NUMBITS(1) [],

        /// IRQ exception vector address mode
        VE OFFSET(24) NUMBITS(1) [],

        // [23:22] undefined, should be one

        /// Fast Interrupt enable (should be one)
        FI OFFSET(21) NUMBITS(1) [],

        // [20] undefined, should be zero

        /// Divide by zero exception enable
        DZ OFFSET(19) NUMBITS(1) [],

        // [18] undefined, should be one

        /// MPU background region enable
        BR OFFSET(17) NUMBITS(1) [],

        // [16] undefined, should be one
        // [15] undefined, should be zero

        /// Replacement strategy for I and D caches (round-robin enable)
        RR OFFSET(14) NUMBITS(1) [
            RandomReplacement = 0,
            RoundRobinReplacement = 1,
        ],

        /// Exception vector location
        V OFFSET(13) NUMBITS(1) [
            Normal = 0,
            High = 1,
        ],

        /// L1 instruction cache enable
        I OFFSET(12) NUMBITS(1) [],

        /// Branch prediction enabled (should be one)
        Z OFFSET(11) NUMBITS(1) [],

        /// SWP and SWPB (swap word/byte to+from memory) enable
        SW OFFSET(10) NUMBITS(1) [],

        // [9:7] undefined, should be zero
        // [6:3] undefined, should be one

        /// L1 data cache enable
        C OFFSET(2) NUMBITS(1) [],

        /// Strict data alignment fault checking enable
        A OFFSET(1) NUMBITS(1) [],

        /// MPU enable
        M OFFSET(0) NUMBITS(1) [],
    ]
];

pub struct Reg;

impl_readable_for_coprocessor_register!(Reg: SCTLR::Register, p15, 0, c1, c0, 0);
impl_writeable_for_coprocessor_register!(volatile Reg: SCTLR::Register, p15, 0, c1, c0, 0);

pub const SCTLR: Reg = Reg;
