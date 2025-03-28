pub static mut BOOT_TIME: u32 = 0;

pub fn delay_seconds(sec: u32) {
    const SYS_TIMER_BASE: usize = 0x3F003000;
    const SYS_TIMER_CLO: *mut u32 = (SYS_TIMER_BASE + 0x4) as *mut u32;
    let start = unsafe { core::ptr::read_volatile(SYS_TIMER_CLO) };
    let target = start.wrapping_add(sec * 1_000_000);
    while unsafe { core::ptr::read_volatile(SYS_TIMER_CLO) } < target {}
}
