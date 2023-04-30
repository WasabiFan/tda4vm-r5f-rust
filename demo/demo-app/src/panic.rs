use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: Is this panic handler actually invoked? Make sure any unwinding is safe.
    // Note: we could be in any privilege level
    // TODO: panic logging
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
