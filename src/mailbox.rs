use core::ptr;

const MAILBOX_BASE: usize = 0x3F00B880;
const MAILBOX_READ: *mut u32 = (MAILBOX_BASE + 0x00) as *mut u32;
const MAILBOX_STATUS: *mut u32 = (MAILBOX_BASE + 0x18) as *mut u32;
const MAILBOX_WRITE: *mut u32 = (MAILBOX_BASE + 0x20) as *mut u32;
const MAILBOX_EMPTY: u32 = 0x40000000;
const MAILBOX_FULL: u32 = 0x80000000;

pub fn mailbox_read(channel: u8) -> u32 {
    loop {
        while unsafe { ptr::read_volatile(MAILBOX_STATUS) } & MAILBOX_EMPTY != 0 {}
        let data = unsafe { ptr::read_volatile(MAILBOX_READ) };
        if (data & 0xF) == channel as u32 {
            return data >> 4;
        }
    }
}

pub fn mailbox_write(channel: u8, data: u32) {
    while unsafe { ptr::read_volatile(MAILBOX_STATUS) } & MAILBOX_FULL != 0 {}
    unsafe {
        ptr::write_volatile(MAILBOX_WRITE, (data << 4) | (channel as u32));
    }
}

pub const MAILBOX_CHANNEL_PROP: u8 = 8;

#[repr(align(16))]
pub struct MailboxBuffer {
    pub buffer: [u32; 8],
}

pub static mut MAILBOX_BUFFER: MailboxBuffer = MailboxBuffer {
    buffer: [
        32,         // Total size in bytes.
        0,          // Request code.
        0x00040001, // Tag: Get framebuffer.
        8,          // Value buffer size.
        8,          // Request/response size.
        800,        // Desired width; becomes framebuffer address.
        600,        // Desired height; becomes pitch.
        0,          // End tag.
    ],
};

pub fn get_framebuffer_address() -> Option<(u32, u32)> {
    unsafe {
        let buffer_ptr = &MAILBOX_BUFFER.buffer as *const u32 as u32;
        mailbox_write(MAILBOX_CHANNEL_PROP, buffer_ptr >> 4);
        let response = mailbox_read(MAILBOX_CHANNEL_PROP);
        if response == (buffer_ptr >> 4) && MAILBOX_BUFFER.buffer[1] == 0x80000000 {
            let fb_addr = MAILBOX_BUFFER.buffer[5] & 0x3FFFFFFF;
            let pitch = MAILBOX_BUFFER.buffer[6];
            Some((fb_addr, pitch))
        } else {
            None
        }
    }
}

const GET_ARM_MEMORY_TAG: u32 = 0x00010005;

#[repr(align(16))]
pub struct RamInfoBuffer {
    pub buffer: [u32; 8],
}

pub static mut RAM_INFO_BUFFER: RamInfoBuffer = RamInfoBuffer {
    buffer: [
        32,                 // Total size in bytes.
        0,                  // Request code.
        GET_ARM_MEMORY_TAG, // Tag: Get ARM Memory.
        8,                  // Value buffer size.
        8,                  // Request/response size.
        0,                  // ARM memory base address (returned).
        0,                  // ARM memory size (returned).
        0,                  // End tag.
    ],
};

pub fn get_ram_info() -> Option<(u32, u32)> {
    unsafe {
        let buffer_ptr = &RAM_INFO_BUFFER.buffer as *const u32 as u32;
        mailbox_write(MAILBOX_CHANNEL_PROP, buffer_ptr >> 4);
        let response = mailbox_read(MAILBOX_CHANNEL_PROP);
        if response == (buffer_ptr >> 4) && RAM_INFO_BUFFER.buffer[1] == 0x80000000 {
            let base = RAM_INFO_BUFFER.buffer[5];
            let size = RAM_INFO_BUFFER.buffer[6];
            Some((base, size))
        } else {
            None
        }
    }
}
