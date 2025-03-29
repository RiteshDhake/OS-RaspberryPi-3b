// In src/bootloader.rs or at the top of main.rs
use crate::uart::Uart;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn rust_print_el2() {
    // Minimal printing - UART might not be fully initialized yet.
    // You can use the UART functions if you ensure they are safe to call here.
    let mut uart = Uart;
    let _ = write!(&mut uart, "########## Privilege level : EL1 ##########\n");
}
