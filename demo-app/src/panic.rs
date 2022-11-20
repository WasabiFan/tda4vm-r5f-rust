use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use crate::trace_buffers::CircularTraceBuffer;

#[link_section = ".log_shared_mem"]
pub static mut PANIC_LOG: CircularTraceBuffer<256> = CircularTraceBuffer::new();

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Note: we could be in any privilege level
    unsafe {
        writeln!(PANIC_LOG, "{}", info).unwrap();
    }

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
