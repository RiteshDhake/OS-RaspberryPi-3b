use core::fmt::Write;
use core::str;
use core::arch::asm;
use core::ptr;
use crate::uart;
use crate::mailbox::{get_ram_info, get_framebuffer_address};
use crate::gpio::GPIO;
use crate::util::delay_seconds;
use crate::hdmi::{init_hdmi,draw_text};


pub fn print_logo(uart: &mut uart::Uart) {
    uart::uart_send_string(b"   ________________________________________________________  \n");
    uart::uart_send_string(b"  /________________________________________________________| \n");
    uart::uart_send_string(b" | ##    ##  ######    ##     ##     ## ##       #######   | \n");
    uart::uart_send_string(b" | ##   ##   ##   ##   ##     ##  ##       ##  ##          | \n");
    uart::uart_send_string(b" | ##  ##    ##   ##   ##     ##  ##       ##  ##          | \n");
    uart::uart_send_string(b" | ## #      #####     ## ### ##  ##       ##     ###      | \n");
    uart::uart_send_string(b" | ##  ##    ##   ##   ##     ##  ##       ##        ###   | \n");
    uart::uart_send_string(b" | ##   ##   ##    ##  ##     ##  ##       ##          ##  | \n");
    uart::uart_send_string(b" | ##    ##  ##     ## ##     ##     ## ##      ########   | \n");
    uart::uart_send_string(b" |_________________________________________________________| \n");
    uart::uart_send_string(b" |________________________________________________________/  \n");
    uart::uart_send_string(b"     K         R          H           O            S         \n");
    uart::uart_send_string(b"------------------------v 0.1.0----------------------------- \n");
}

pub fn print_logo_hdmi(fb_addr: u32, pitch: u32,x: u32, y: u32,fg_color: u32, bg_color: Option<u32>){
    unsafe{
    draw_text(fb_addr, pitch,100, 100, "   ________________________________________________________  \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 120, "  /________________________________________________________| \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 140, " | ##    ##  ######    ##     ##     ## ##       #######   | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 160, " | ##   ##   ##   ##   ##     ##  ##       ##  ##          | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 180, " | ##  ##    ##   ##   ##     ##  ##       ##  ##          | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 200, " | ## #      #####     ## ### ##  ##       ##     ###      | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 220, " | ##  ##    ##   ##   ##     ##  ##       ##        ###   | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 240, " | ##   ##   ##    ##  ##     ##  ##       ##          ##  | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 260, " | ##    ##  ##     ## ##     ##     ## ##      ########   | \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 280, " |_________________________________________________________| \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 300, " |________________________________________________________/  \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 320, "     K         R          H           O            S         \n", 0xFFFFFF, Some(0x000000));
    draw_text(fb_addr, pitch,100, 340, "------------------------v 0.1.0----------------------------- \n", 0xFFFFFF, Some(0x000000));
    }
}

pub unsafe fn draw_square(fb_addr: u32, pitch: u32, x: u32, y: u32, color: u32) {
    // Convert pitch (bytes) to pitch (pixels) assuming 32-bit pixels.
    let start_x = x;
    let start_y = y;
    let pitch_pixels = 1024;
    let fb_ptr = fb_addr as *mut u32;
    for y in start_y..(start_y+100){
        for x in start_x..(start_x+100){

            let index = ((y * pitch_pixels)+ x) as usize;
            fb_ptr.add(index).write_volatile(color);
        }
    } 
}


pub fn process_command(cmd: &str, uart: &mut uart::Uart) {
    if cmd == "help" {
        uart::uart_send_string(b"Available commands: help, meminfo, raminfo, demo, gpio xx, showgpio xx, memdump, regdump, reboot, uptime , init_hdmi , togglegpio xx\n");
    } else if cmd == "meminfo" {
        // Print memory layout using linker symbols.
        unsafe {
            extern "C" {
                static _sdata: u8;
                static _edata: u8;
                static _sbss: u8;
                static _ebss: u8;
                static _end: u8;
                static _stack_top: u8;
            }
            let sdata = &_sdata as *const u8 as usize;
            let edata = &_edata as *const u8 as usize;
            let sbss  = &_sbss as *const u8 as usize;
            let ebss  = &_ebss as *const u8 as usize;
            let kernel_end = &_end as *const u8 as usize;
            let stack_top = &_stack_top as *const u8 as usize;
            let data_size = edata - sdata;
            let bss_size = ebss - sbss;
            let _ = write!(uart, "Data: {:#x} - {:#x} ({} bytes)\n", sdata, edata, data_size);
            let _ = write!(uart, "BSS:  {:#x} - {:#x} ({} bytes)\n", sbss, ebss, bss_size);
            let _ = write!(uart, "Kernel end: {:#x}\n", kernel_end);
            let _ = write!(uart, "Stack top: {:#x}\n", stack_top);
        }
    } else if cmd == "raminfo" {
        if let Some((base, size)) = get_ram_info() {
            let _ = write!(uart, "RAM base: {:#x}, size: {:#x}\n", base, size);
        } else {
            uart::uart_send_string(b"Failed to get RAM info.\n");
        }
    } else if cmd == "demo" {
        // Use the system timer (free-running at ~1MHz) for timing.
        const SYS_TIMER_BASE: usize = 0x3F003000;
        const SYS_TIMER_CLO: *mut u32 = (SYS_TIMER_BASE + 0x4) as *mut u32;
        let start = unsafe { core::ptr::read_volatile(SYS_TIMER_CLO) };
        for _ in 0..50_000 {
            unsafe { asm!("nop", options(nomem, nostack)); }
        }
        let end = unsafe { core::ptr::read_volatile(SYS_TIMER_CLO) };
        let elapsed = if end >= start {
            end - start
        } else {
            (0xFFFFFFFF - start) + end + 1
        };
        let _ = write!(uart, "Loop ran for {} microseconds\n", elapsed);
    } else if cmd.starts_with("gpio ") {
          // Expecting: "gpio XX" where XX is the GPIO pin number to toggle.
          let arg = cmd[5..].trim();
          match arg.parse::<u32>() {
              Ok(pin) => {
                  // Set the pin as output and then toggle it.
                  if !crate::gpio::GPIO::check_pin_no(pin) {
                        uart::uart_send_string(b"Invalid pin number. Pin must be less than 28.\n");
                        return;
                   } 
                  crate::gpio::GPIO::set_output(pin);
                  let _ = write!(uart, "Toggling GPIO pin {}...\n", pin);
                  for _ in 0..10 {
                      crate::gpio::GPIO::set(pin);
                      let level = crate::gpio::GPIO::read_level(pin);
                      let _ = write!(uart, "GPIO pin {} level: {}\n", pin, if level { "HIGH" } else { "LOW" });
                    //   for _ in 0..5_000_000 {
                    //       unsafe { core::arch::asm!("nop", options(nomem, nostack)); }
                    //   }
                        crate::util::delay_seconds(2);
                      crate::gpio::GPIO::clear(pin);
                      let level = crate::gpio::GPIO::read_level(pin);
                      let _ = write!(uart, "GPIO pin {} level: {}\n", pin, if level { "HIGH" } else { "LOW" });
                    //   for _ in 0..5_000_000 {
                    //       unsafe { core::arch::asm!("nop", options(nomem, nostack)); }
                    //   }
                        crate::util::delay_seconds(2);
                  }
                  crate::uart::uart_send_string(b"GPIO toggle complete.\n");
              },
              Err(_) => {
                  crate::uart::uart_send_string(b"Invalid GPIO pin number.\n");
              }
          }
        uart::uart_send_string(b"GPIO demo complete.\n");
    } else if cmd.starts_with("showgpio "){
        // Expecting: "showgpio XX" where XX is the GPIO pin number to read.
        let arg = cmd[9..].trim();
        match arg.parse::<u32>() {
            Ok(pin) => {
                if !crate::gpio::GPIO::check_pin_no(pin) {
                    uart::uart_send_string(b"Invalid pin number. Pin must be less than 28.\n");
                    return;
                }
                let _ = write!(uart, "Reading GPIO pin {} state...\n", pin);
                let level = crate::gpio::GPIO::read_level(pin);
                let _ = write!(uart, "GPIO pin {} level: {}\n", pin, if level { "HIGH" } else { "LOW" });
            },
            Err(_) => {
                crate::uart::uart_send_string(b"Invalid GPIO pin number.\n");
            }
        }
    } else if cmd == "memdump" {
        // Dump 64 bytes starting at address 0x80000 (for example)
        uart::uart_send_string(b"Memory dump at 0x80000:\n");
        let start_addr = 0x80000 as *const u8;
        for i in 0..64 {
            let byte = unsafe { *start_addr.offset(i) };
            let _ = write!(uart, "{:02x} ", byte);
            if (i + 1) % 16 == 0 {
                uart::uart_send_string(b"\n");
            }
        }
    } else if cmd == "regdump" {
        // Dump a couple of registers: CurrentEL and SP.
        let mut current_el: u64;
    let mut sp: u64;
    let mut midr: u64;
    let mut mpidr: u64;
    let mut tpidr_el0: u64;
    let mut cntfrq: u64;
    let mut vbar: u64;
    unsafe {
        core::arch::asm!("mrs {0}, CurrentEL", out(reg) current_el);
        core::arch::asm!("mov {0}, sp", out(reg) sp);
        core::arch::asm!("mrs {0}, MIDR_EL1", out(reg) midr);
        core::arch::asm!("mrs {0}, MPIDR_EL1", out(reg) mpidr);
        core::arch::asm!("mrs {0}, TPIDR_EL0", out(reg) tpidr_el0);
        core::arch::asm!("mrs {0}, CNTFRQ_EL0", out(reg) cntfrq);
        core::arch::asm!("mrs {0}, VBAR_EL1", out(reg) vbar);
    }
    let _ = write!(uart, "CurrentEL: {:#x}\n", current_el >> 2); // bits [3:2] hold the EL.
    let _ = write!(uart, "SP: {:#x}\n", sp);
    let _ = write!(uart, "MIDR_EL1: {:#x}\n", midr);
    let _ = write!(uart, "MPIDR_EL1: {:#x}\n", mpidr);
    let _ = write!(uart, "TPIDR_EL0: {:#x}\n", tpidr_el0);
    let _ = write!(uart, "CNTFRQ_EL0: {:#x}\n", cntfrq);
    let _ = write!(uart, "VBAR_EL1: {:#x}\n", vbar);
    } else if cmd == "uptime" {
        // Uptime based on the system timer.
        const SYS_TIMER_BASE: usize = 0x3F003000;
        const SYS_TIMER_CLO: *mut u32 = (SYS_TIMER_BASE + 0x4) as *mut u32;
        let now = unsafe { core::ptr::read_volatile(SYS_TIMER_CLO) };
        let boot_time = unsafe { crate::util::BOOT_TIME };
        let elapsed = if now >= boot_time { now - boot_time } else { (0xFFFFFFFF - boot_time) + now + 1 };
        // Convert microseconds to seconds (assuming ~1MHz clock).
        let seconds = elapsed as f32 / 1_000_000.0;
        let _ = write!(uart, "Uptime: {:.6} seconds\n", seconds);
    } else if cmd == "reboot" {
        uart::uart_send_string(b"Rebooting...\n");
        reboot();
    } else if cmd =="init_hdmi"{
            // Initialize HDMI output.
    //     match init_hdmi() {
    //     Some((fb_addr, pitch, width, height, depth)) => {
    //         let _ = write!(
    //             uart,
    //             "Framebuffer at {:#x}, pitch: {:#x}, resolution: {}x{}, depth: {}\n",
    //             fb_addr, pitch, width, height, depth
    //         );
 
    //         // For demonstration: fill the framebuffer with red (assuming 32-bit pixel format).
    //         // unsafe {
    //         //     let fb_ptr = fb_addr as *mut u32;
    //         //     for y in 0..height {
    //         //         for x in 0..width {
    //         //             // Calculate offset using the pitch.
    //         //             fb_ptr.add((y * pitch + x) as usize).write_volatile(0x00FF0000);
    //         //         }
    //         //     }
    //         // }
    //         uart::uart_send_string(b"HDMI initialized and framebuffer drawn.\n");
    //         // Draw a red square of 100x100 pixels in the center of the screen.
    //         unsafe {
    //             draw_red_square(fb_addr, pitch, width, height, 100);
    //         }
    //         uart::uart_send_string(b"Red square drawn on framebuffer.\n");
    //     }
    //     None => {
    //         uart::uart_send_string(b"HDMI initialization failed.\n");
    //     }
    // }
        match init_hdmi() {
            Some((fb_addr, pitch, width, height, depth)) => {
                let _ = write!(
                    uart,
                    "Framebuffer at {:#x}, pitch: {:#x}, resolution: {}x{}, depth: {}\n",
                    fb_addr, pitch, width, height, depth
                );
                // Draw a red square (example) by filling the framebuffer.
                unsafe {
                    //draw_red_square(fb_addr, pitch, width, height, 200);
                    //draw_square(fb_addr, pitch, 100, 400, 0x000000FF);
                    //draw_square(fb_addr, pitch, 200, 400, 0x0000FF00);
                    //draw_text(fb_addr, pitch,100, 100, "Hi Ritesh Welcomes you", 0xFFFFFF, Some(0x000000));
                    print_logo_hdmi(fb_addr, pitch,100, 100, 0xFFFFFF, Some(0x000000))
                    
                }
                
                // unsafe {
                //     let fb_ptr = fb_addr as *mut u32;
                //     // For demonstration, fill the entire framebuffer with red.
                //     for y in 0..height {
                //         for x in 0..width {
                //             // Assuming pitch is in bytes and 32-bit pixels (pitch in pixels = pitch / 4)
                //             let pixel_index = y * (pitch/4) + x +50000;
                //             fb_ptr.add(pixel_index as usize).write_volatile(0x00FF0000);
                //         }
                //     }
                // }
                uart::uart_send_string(b"HDMI initialized and red square drawn.\n");
            }
            None => {
                uart::uart_send_string(b"HDMI initialization failed.\n");
                }
        }
    }else if cmd.starts_with("togglegpio "){
        let arg = cmd[11..].trim();
        match arg.parse::<u32>() {
            Ok(pin) => {
                if !crate::gpio::GPIO::check_pin_no(pin) {
                    uart::uart_send_string(b"Invalid pin number. Pin must be less than 28.\n");
                    return;
                }
                let _ = write!(uart, "Toggling GPIO pin {} state...\n", pin);
                crate::gpio::GPIO::set_output(pin);
                let level = crate::gpio::GPIO::read_level(pin);
                if level{
                    crate::gpio::GPIO::clear(pin);
                }else{
                    crate::gpio::GPIO::set(pin);
                }
                let level1 = crate::gpio::GPIO::read_level(pin);
                let _ = write!(uart, "GPIO pin {} level: {}\n", pin, if level1 { "HIGH" } else { "LOW" });
            },
            Err(_) => {
                crate::uart::uart_send_string(b"Invalid GPIO pin number.\n");
            }
        }

    }else {
        uart::uart_send_string(b"Unknown command. Type 'help' for available commands.\n");
    }
}

// Reboot function using the watchdog registers.
fn reboot() -> ! {
    const PM_RSTC: *mut u32 = 0x3F10001C as *mut u32;
    const PM_WDOG: *mut u32 = 0x3F100024 as *mut u32;
    const PM_PASSWORD: u32 = 0x5A000000;
    unsafe {
        ptr::write_volatile(PM_WDOG, PM_PASSWORD | 100); // Set timeout
        ptr::write_volatile(PM_RSTC, PM_PASSWORD | 0x20);  // Issue reset command
    }
    loop {}
}
