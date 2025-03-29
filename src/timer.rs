use core::ptr;
use core::fmt::Write;
use crate::uart::Uart;

const TIMER_BASE: usize = 0x3F00B400;
const TIMER_LOAD: *mut u32 = (TIMER_BASE + 0x00) as *mut u32;
const TIMER_VALUE: *mut u32 = (TIMER_BASE + 0x04) as *mut u32;
const TIMER_CTRL: *mut u32 = (TIMER_BASE + 0x08) as *mut u32;
const TIMER_CLR: *mut u32 = (TIMER_BASE + 0x0C) as *mut u32;

const IRQ_BASIC_ENABLE: *mut u32 = 0x3F00B200 as *mut u32;
const IRQ_ENABLE1: *mut u32 = 0x3F00B210 as *mut u32;
const ARM_TIMER_IRQ_BIT: u32 = 1 << 0;

pub fn timer_init() {
    unsafe {
        ptr::write_volatile(TIMER_LOAD, 0x00001000); // 4096 ticks.
        ptr::write_volatile(TIMER_CLR, 0x1);           // Clear pending interrupt.
        ptr::write_volatile(TIMER_CTRL, 0xE2);           // Enable timer, periodic, IRQ enabled, 32-bit.
    }
}

pub fn interrupt_controller_init() {
    unsafe {
        ptr::write_volatile(IRQ_BASIC_ENABLE, ARM_TIMER_IRQ_BIT);
        let current = ptr::read_volatile(IRQ_ENABLE1);
        ptr::write_volatile(IRQ_ENABLE1, current | (1 << 0));
    }
}

pub fn print_timer_value() {
    unsafe {
        let val = ptr::read_volatile(TIMER_VALUE);
        let mut uart = Uart;
        let _ = write!(&mut uart, "Timer value: {:#x}\n", val);
    }
}

#[no_mangle]
pub extern "C" fn rust_irq_handler() {
    unsafe {
        ptr::write_volatile(TIMER_CLR, 0x1);
    }
    crate::uart::uart_send_string(b"\nTimer IRQ triggered!\n");
}
