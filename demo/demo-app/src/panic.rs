use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Note: we could be in any privilege level
    // TODO: panic logging
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
