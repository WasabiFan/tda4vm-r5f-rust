#![no_std]
#![no_main]

mod panic;

use core::{arch::{asm, global_asm}, sync::atomic::{self, Ordering}};

use remoteproc_resource_table::{TraceResourceTypeData, ResourceTableTargetAddress, resource_table, ZeroBytes};

#[no_mangle]
#[link_section = ".log_shared_mem"]
static mut DEBUG_LOG: [ u8; 100 ] = [ 0; 100 ];


#[cfg(not(target_pointer_width = "32"))]
compile_error!("Requires 32-bit pointers");

resource_table![
    TraceResourceTypeData {
        device_address: ResourceTableTargetAddress(unsafe {  &DEBUG_LOG as *const u8 }),
        length: 100,
        _reserved: ZeroBytes::new(),
        name: {
            let mut x = [0; 32];

            x[0] = b'a';
            x[1] = b'b';
            x[2] = b'c';

            x
        }
    }
];

// TODO: did TI add any custom interrupt hardware?
// Vectored ISR handler table. Ref: https://developer.arm.com/documentation/ddi0460/d/Programmers-Model/Exceptions/Exception-vectors?lang=en
// Note: the ELF entry point needs to also refer to this table, so we declare _start here since the linker seems to like that.
global_asm!(r#"
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
"#);

#[no_mangle]
unsafe extern "C" fn _rt_isr_default() {
    loop {
        // TODO: expose/handle exception in some way
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[no_mangle]
pub unsafe extern "C" fn _entry() -> ! {
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

    // Initialize stack and zero lr
    asm!(
        "ldr sp, =__StackTop",
        "mov lr, #0"
    );

    // Initialize supervisor PSR
    asm!(
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
    // TODO: consider how trace buffer should be used -- it's limited size, how does overflow work?
    // println!("Hello, world!");

    unsafe {
        DEBUG_LOG[0] = b'h';
        DEBUG_LOG[1] = b'i';
        DEBUG_LOG[4] = b'\n';
        DEBUG_LOG[5] = 0;
    }

    let mut x = 0usize;
    loop {
        // Dummy busy loop to prevent termination and provide something to observe in the debugger
        x = x.wrapping_add(1);
    }
}
