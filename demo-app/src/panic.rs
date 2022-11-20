use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

#[no_mangle]
#[link_section = ".log_shared_mem"]
static mut PANIC_LOG: [u8; 256] = [0; 256];

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    writeln!("{}", info, PANIC_LOG);

    loop {
        // TODO: implement a better panic handler
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
