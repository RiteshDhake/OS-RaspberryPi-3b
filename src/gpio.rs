use core::ptr;

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;
const GPIO_SETO: u32  = 0x3F20_001C;
const GPIO_CLRO: u32  = 0x3F20_0028;

const GPIO_LEV0: u32 = 0x3F20_0034;
const GPIO_LEV1: u32 = 0x3F20_0038;


pub struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {
        let reg_addr = match pin / 10 {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Invalid pin number"),
        };
        let shift = (pin % 10) * 3;
        unsafe {
            let mut val = ptr::read_volatile(reg_addr as *mut u32);
            val &= !(0b111 << shift);
            val |= (0b001 << shift);
            ptr::write_volatile(reg_addr as *mut u32, val);
        }
    }

    pub fn set(pin: u32) {
        unsafe {
            let mut val = ptr::read_volatile(GPIO_SETO as *mut u32);
            val |= 1 << pin;
            ptr::write_volatile(GPIO_SETO as *mut u32, val);
        }
    }

    pub fn clear(pin: u32) {
        unsafe {
            let mut val = ptr::read_volatile(GPIO_CLRO as *mut u32);
            val |= 1 << pin;
            ptr::write_volatile(GPIO_CLRO as *mut u32, val);
        }
    }



    // Helper read functions:

    pub fn read_level(pin: u32) -> bool {
        // For pins 0..31, use GPLEV0; for 32 and above, use GPLEV1.
        let reg = if pin < 32 { GPIO_LEV0 } else { GPIO_LEV1 };
        unsafe {
            let level = ptr::read_volatile(reg as *const u32);
            (level & (1 << (pin % 32))) != 0
        }
    }

    pub fn read_fsel0() -> u32 {
        unsafe { ptr::read_volatile(GPIO_FSEL0 as *const u32) }
    }
    pub fn read_fsel1() -> u32 {
        unsafe { ptr::read_volatile(GPIO_FSEL1 as *const u32) }
    }
    pub fn read_fsel2() -> u32 {
        unsafe { ptr::read_volatile(GPIO_FSEL2 as *const u32) }
    }
    pub fn read_seto() -> u32 {
        unsafe { ptr::read_volatile(GPIO_SETO as *const u32) }
    }
    pub fn read_clro() -> u32 {
        unsafe { ptr::read_volatile(GPIO_CLRO as *const u32) }
    }
    pub fn check_pin_no(pin: u32)->bool{
        pin < 28
     }
}
