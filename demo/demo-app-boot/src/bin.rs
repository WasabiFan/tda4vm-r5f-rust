#![no_std]
#![no_main]

mod mpu;
mod panic;
mod trace_buffers;

use core::{
    arch::{asm, global_asm},
    fmt::Write,
    sync::atomic::{self, Ordering},
};

use mpu::{MpuRegion, MpuRegionSize};
use panic::PANIC_LOG;
use trace_buffers::CircularTraceBuffer;

use cortex_r5_pac::registers::Readable;
use remoteproc_resource_table::{
    packing::{ResourceTableTargetAddress, ZeroBytes},
    resource_table,
    trace::TraceResourceTypeData,
    vdev::{RpmsgFeatures, VdevResourceTypeData, VdevResourceVringDescriptor, VirtIODeviceId},
};

extern "C" {
    pub fn run_me_from_ddr_too(x: u32) -> u32;
    pub fn get_reg_from_ddr() -> u32;
}

#[link_section = ".log_shared_mem"]
static mut DEBUG_LOG: CircularTraceBuffer<4096> = CircularTraceBuffer::new();

#[cfg(not(target_pointer_width = "32"))]
compile_error!("Requires 32-bit pointers");

resource_table![
    static debug_log: TraceResourceTypeData =
        TraceResourceTypeData::from_buffer("debug", unsafe { &DEBUG_LOG.buffer });
    static panic_log: TraceResourceTypeData =
        TraceResourceTypeData::from_buffer("panic", unsafe { &PANIC_LOG.buffer });
    static rpmesg_log: VdevResourceTypeData<2> = VdevResourceTypeData {
        id: VirtIODeviceId::VIRTIO_ID_RPMSG,
        notifyid: 0, // TODO
        dfeatures: RpmsgFeatures::VIRTIO_RPMSG_F_NS.bits(),
        gfeatures: 0,  // To be populated by host
        config_len: 0, // Config space not supported
        status: 0,
        num_of_vrings: 2,
        _reserved: ZeroBytes::new(),
        vring: [
            VdevResourceVringDescriptor {
                device_address: ResourceTableTargetAddress::ADDR_ANY,
                align: 0x1000,
                num: 8,
                notifyid: 1,
                physical_address: ResourceTableTargetAddress::ADDR_ANY,
            },
            VdevResourceVringDescriptor {
                device_address: ResourceTableTargetAddress::ADDR_ANY,
                align: 0x1000,
                num: 8,
                notifyid: 2,
                physical_address: ResourceTableTargetAddress::ADDR_ANY,
            },
        ],
    };
];

// TODO: did TI add any custom interrupt hardware?
// Vectored ISR handler table. Ref: https://developer.arm.com/documentation/ddi0460/d/Programmers-Model/Exceptions/Exception-vectors?lang=en
// Note: the ELF entry point needs to also refer to this table, so we declare _start here since the linker seems to like that.
global_asm!(
    r#"
    .section .isr_vector, "ax"
    .type _start, %function
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

    let mpu_handle = unsafe { mpu::DisabledMpuHandle::new() };

    let mpu_handle = mpu_handle.enable(
        &[
            MpuRegion {
                base_address: 0x0,
                size: MpuRegionSize::from_log2_bytes(15).unwrap(),
                attributes: mpu::MpuRegionAttributes {
                    execute_never: false,
                    access_permissions: mpu::MpuRegionAccessPermissions::AllReadWrite,
                    coherence_configuration: mpu::MpuRegionCoherenceConfiguration::Normal(
                        mpu::MpuRegionCachePolicy::AllDomains(
                            mpu::MpuRegionShareabilityDomainCachePolicy::NonCacheable,
                        ),
                        mpu::MpuRegionShareability::NonShareable,
                    ),
                    subregion_disable_mask: 0,
                },
            },
            MpuRegion {
                base_address: 0x8000_0000,
                size: MpuRegionSize::from_log2_bytes(31).unwrap(),
                attributes: mpu::MpuRegionAttributes {
                    execute_never: false,
                    access_permissions: mpu::MpuRegionAccessPermissions::AllReadWrite,
                    coherence_configuration: mpu::MpuRegionCoherenceConfiguration::Normal(
                        mpu::MpuRegionCachePolicy::AllDomains(
                            mpu::MpuRegionShareabilityDomainCachePolicy::NonCacheable,
                        ),
                        mpu::MpuRegionShareability::NonShareable,
                    ),
                    subregion_disable_mask: 0,
                },
            },
        ],
        false,
    );

    unsafe {
        writeln!(DEBUG_LOG, "We survived the MPU!").unwrap();
    }

    let val = run_me_from_ddr(12);

    unsafe {
        writeln!(DEBUG_LOG, "Boot app DDR code returned: {:x}", val).unwrap();
    }

    unsafe {
        let val = run_me_from_ddr_too(12);
        writeln!(DEBUG_LOG, "Main app DDR code returned: {:x}", val).unwrap();
    }

    unsafe {
        // Demo invoking Rust library in main app, with library also present in boot
        let main_app_val = get_reg_from_ddr();
        let boot_app_val = cortex_r5_pac::registers::system_control::SCTLR.get();
        writeln!(
            DEBUG_LOG,
            "Main app fetched SCTLR: {:x}, observed value is: {:x}",
            main_app_val, boot_app_val
        )
        .unwrap();
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

#[link_section = ".ddr.text"]
#[inline(never)]
fn run_me_from_ddr(x: u32) -> u32 {
    let reg: u32;
    unsafe {
        asm!("mov {reg}, pc", reg = out(reg) reg);
        writeln!(DEBUG_LOG, "Hello from DDR, pc = {:x}", reg).unwrap();
    }
    reg
}
