
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // TODO: implement a better panic handler
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
