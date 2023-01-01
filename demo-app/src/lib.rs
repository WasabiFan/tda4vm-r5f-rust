#![no_std]

use core::arch::asm;

use cortex_r5_pac::registers::{system_control::SCTLR, Readable};

mod panic;

#[no_mangle]
#[inline(never)]
pub extern "C" fn run_me_from_ddr_too(x: u32) -> u32 {
    let reg: u32;
    unsafe {
        asm!("mov {reg}, pc", reg = out(reg) reg);
        // TODO: establish resources in main app
        // writeln!(DEBUG_LOG, "Hello from DDR, pc = {:x}", reg).unwrap();
    }
    reg
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn get_reg_from_ddr() -> u32 {
    let val = SCTLR.get();
    unsafe {
        asm!("nop");
    }
    val
}
