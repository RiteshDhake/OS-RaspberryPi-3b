use crate::uart; 
use crate::scheduler;
use crate::uart::Uart;
use crate::gpio::GPIO;
use crate::mailbox::get_ram_info;
use crate::timer;
use core::arch::asm;
use core::fmt::Write;

/// Run the self-test tasks forever using cooperative scheduling.
/// This function never returns.
pub fn run_selftest(){
    unsafe {
        // Register test tasks.
        scheduler::add_task(test_gpio);
        scheduler::add_task(test_interrupt);
        scheduler::add_task(test_mailbox);
        // Run all registered tasks in round-robin.
        scheduler::run_tasks();
    }
}

/// Task to test GPIO functionality: toggles a given pin and prints status.
fn test_gpio() {
    let uart = Uart;
    uart::uart_send_string(b"[Selftest] Starting GPIO test on pin 16...\n");
    // For this test, we use GPIO pin 16.
    GPIO::set_output(16);
    GPIO::set(16);
    for _ in 0..1_000_000 {
        unsafe { asm!("nop", options(nomem, nostack)); }
    }
    GPIO::clear(16);
    uart::uart_send_string(b"[Selftest] GPIO test complete.\n");
}

/// Task to test timer interrupts.
/// For example, it simply waits (yielding control) so that the IRQ handler can fire.
fn test_interrupt() {
    let uart = Uart;
    uart::uart_send_string(b"[Selftest] Testing timer interrupts...\n");
    // Let the timer run and yield.
    for _ in 0..5_000_000 {
        unsafe { asm!("nop", options(nomem, nostack)); }
    }
    timer::print_timer_value();
    uart::uart_send_string(b"[Selftest] Timer interrupt test complete (check IRQ logs).\n");
}

/// Task to test the mailbox functionality (e.g. get RAM info).
fn test_mailbox() {
    let mut uart = Uart;
    uart::uart_send_string(b"[Selftest] Checking mailbox for RAM info...\n");
    if let Some((base, size)) = get_ram_info() {
        let _ = write!(uart, "[Selftest] RAM base: {:#x}, size: {:#x}\n", base, size);
    } else {
        uart::uart_send_string(b"[Selftest] Failed to retrieve RAM info.\n");
    }
}
