#![no_std]
#![no_main]

mod panic;
mod trace_buffers;

use core::{
    arch::{asm, global_asm},
    fmt::Write,
    sync::atomic::{self, Ordering},
};

use panic::PANIC_LOG;
use trace_buffers::CircularTraceBuffer;

use remoteproc_resource_table::{
    fixed_length_str, resource_table, ResourceTableTargetAddress, TraceResourceTypeData, ZeroBytes,
};

#[link_section = ".log_shared_mem"]
static mut DEBUG_LOG: CircularTraceBuffer<256> = CircularTraceBuffer::new();

#[cfg(not(target_pointer_width = "32"))]
compile_error!("Requires 32-bit pointers");

resource_table![
    static debug_log: TraceResourceTypeData = TraceResourceTypeData {
        device_address: ResourceTableTargetAddress(unsafe { &DEBUG_LOG.buffer as *const u8 }),
        length: CircularTraceBuffer::length(unsafe { &DEBUG_LOG }) as u32,
        _reserved: ZeroBytes::new(),
        name: fixed_length_str("debug"),
    };
    static panic_log: TraceResourceTypeData = TraceResourceTypeData {
        device_address: ResourceTableTargetAddress(unsafe { &PANIC_LOG.buffer as *const u8 }),
        length: CircularTraceBuffer::length(unsafe { &PANIC_LOG }) as u32,
        _reserved: ZeroBytes::new(),
        name: fixed_length_str("panic"),
    };
];

// TODO: did TI add any custom interrupt hardware?
// Vectored ISR handler table. Ref: https://developer.arm.com/documentation/ddi0460/d/Programmers-Model/Exceptions/Exception-vectors?lang=en
// Note: the ELF entry point needs to also refer to this table, so we declare _start here since the linker seems to like that.
global_asm!(
    r#"
    .section .isr_vector, "ax"
    .type start, %function
    .type isr_vector, %function
    .global _start
    .global isr_vector
    start:
    isr_vector:
        ldr pc,=_entry                          /* 0x00 */
        ldr pc,=_rt_tramp_undefined_instr       /* 0x04 */
        ldr pc,=_rt_tramp_software_irq          /* 0x08 */
        ldr pc,=_rt_tramp_abort_prefetch        /* 0x0C */
        ldr pc,=_rt_tramp_abort_data            /* 0x10 */
        nop                                     /* unused */
        ldr pc,=_rt_tramp_irq                   /* 0x18 */
        ldr pc,=_rt_tramp_fiq                   /* 0x1C */

    .section .text
    _rt_tramp_undefined_instr:
    _rt_tramp_software_irq:
    _rt_tramp_abort_prefetch:
    _rt_tramp_abort_data:
    _rt_tramp_irq:
    _rt_tramp_fiq:
        // Temporary (?) hack: switch back to supervisor mode to coopt supervisor stack
        cps #0x13
        b _rt_isr_default
    "#
);

#[no_mangle]
unsafe extern "C" fn _rt_isr_default() {
    loop {
        // TODO: expose/handle exception in some way
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
unsafe extern "C" fn _entry() -> ! {
    // Initially in Supervisor mode
    // Zero GPRs
    asm!(
        "mov r0, #0",
        "mov r1, #0",
        "mov r2, #0",
        "mov r3, #0",
        "mov r4, #0",
        "mov r5, #0",
        "mov r6, #0",
        "mov r7, #0",
        "mov r8, #0",
        "mov r9, #0",
        "mov r10, #0",
        "mov r11, #0",
        "mov r12, #0",
    );

    asm!(
        // Initialize stack and zero lr
        "ldr sp, =__StackTop",
        "mov lr, #0"
    );

    asm!(
        // Initialize supervisor PSR
        "mrs r1, cpsr",
        "msr spsr_cxsf, r1"
    );

    // TODO: initialize banked registers for FIQ, IRQ, Abort, Undefined instruction, System
    // TODO: initialize FPU
    // TODO: enable ECC
    // TODO: configure caches and DDR
    // TODO: verify that remoteproc loader initializes all important memory regions
    // TODO: identify any clocks/PLLs that need to be configured

    main()
}

fn main() -> ! {
    unsafe {
        // TODO: implement critical sections
        writeln!(DEBUG_LOG, "Hello world!").unwrap();
    }

    let mut x = 0usize;
    loop {
        // Dummy busy loop to prevent termination and provide something to observe in the debugger
        x = x.wrapping_add(1);

        if x % 50_000_000 == 0 {
            unsafe {
                writeln!(DEBUG_LOG, "Iter {}", x).unwrap();
            };
        }

        if x > 200_000_000 {
            panic!("panik");
        }
    }
}
