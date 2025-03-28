#![no_std]
#![no_main]
#![feature(asm)]

use core::arch::asm;
use core::str;
use core::fmt::Write;

mod uart;
mod gpio;
mod timer;
mod mailbox;
mod commands;
mod util;
mod bootloader;
mod scheduler;
mod selftest;
mod hdmi;


use uart::{uart_init, uart_send_string, uart_receive, Uart};
use timer::{timer_init, interrupt_controller_init};
use hdmi::init_hdmi;


// Global command buffer.
static mut CMD_BUFFER: [u8; 128] = [0; 128];
static mut CMD_BUFFER_INDEX: usize = 0;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart_init();
    timer_init();
    interrupt_controller_init();

    // Record boot time using the system timer.
    const SYS_TIMER_BASE: usize = 0x3F003000;
    const SYS_TIMER_CLO: *mut u32 = (SYS_TIMER_BASE + 0x4) as *mut u32;
    unsafe {
        util::BOOT_TIME = core::ptr::read_volatile(SYS_TIMER_CLO);
    }

    let mut uart = Uart;
    commands::print_logo(&mut uart);
    let _ = write!(&mut uart, "Welcome to KRHOS\n");
    uart_send_string(b"Running self-test...\n");
    selftest::run_selftest();
    uart_send_string(b"\n>>");




    loop {
        let input_byte = uart_receive();
        unsafe {
            if input_byte == 0x7F {
                if CMD_BUFFER_INDEX > 0 {
                    CMD_BUFFER_INDEX = CMD_BUFFER_INDEX.saturating_sub(1);
                    uart_send_string(b"\x08 \x08");
                }
            } else if input_byte == b'\r' || input_byte == b'\n' {
                uart_send_string(b"\n");
                if let Ok(cmd_str) = str::from_utf8(&CMD_BUFFER[..CMD_BUFFER_INDEX]) {
                    commands::process_command(cmd_str, &mut uart);
                    uart_send_string(b">>");
                } else {
                    uart_send_string(b"Invalid UTF-8 input.\n");
                }
                CMD_BUFFER_INDEX = 0;
            } else {
                if CMD_BUFFER_INDEX < 127 {
                    CMD_BUFFER[CMD_BUFFER_INDEX] = input_byte;
                    CMD_BUFFER_INDEX += 1;
                    uart_send_string(&[input_byte]);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
