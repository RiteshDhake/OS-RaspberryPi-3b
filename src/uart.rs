use core::ptr;
use core::fmt::{self, Write};
use core::arch::asm;

const UART0_DR: *mut u32   = 0x3F201000 as *mut u32;
const UART0_FR: *mut u32   = 0x3F201018 as *mut u32;
const UART0_IBRD: *mut u32 = 0x3F201024 as *mut u32;
const UART0_FBRD: *mut u32 = 0x3F201028 as *mut u32;
const UART0_LCRH: *mut u32 = 0x3F20102C as *mut u32;
const UART0_CR: *mut u32   = 0x3F201030 as *mut u32;

pub struct Uart;

pub fn uart_init() {
    unsafe {
        // Disable UART.
        ptr::write_volatile(UART0_CR, 0);
        let baud_div = 48_000_000 / (16 * 115_200);
        ptr::write_volatile(UART0_IBRD, baud_div & 0xFFFF);
        ptr::write_volatile(
            UART0_FBRD,
            (((48_000_000 % (16 * 115_200)) * 64 + 115_200 / 2) / 115_200) as u32,
        );
        // 8-bit, no parity, 1 stop bit.
        ptr::write_volatile(UART0_LCRH, 0x60);
        // Enable TX, RX, and UART.
        ptr::write_volatile(UART0_CR, 0x301);
    }
}

pub fn uart_send(c: u8) {
    unsafe {
        while ptr::read_volatile(UART0_FR) & (1 << 5) != 0 {}
        ptr::write_volatile(UART0_DR, c as u32);
    }
}

pub fn uart_send_string(s: &[u8]) {
    for &byte in s {
        uart_send(byte);
    }
}

pub fn uart_receive() -> u8 {
    unsafe {
        while ptr::read_volatile(UART0_FR) & (1 << 4) != 0 {
            asm!("nop");
        }
        ptr::read_volatile(UART0_DR) as u8
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        uart_send_string(s.as_bytes());
        Ok(())
    }
}
