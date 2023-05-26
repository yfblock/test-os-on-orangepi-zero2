use core::ptr::read_volatile;

use crate::{print, println};

const RTC_BASE: usize = 0x7000_0000;

// fn real_time() -> usize {

// }

fn print_timestamp() {
    unsafe {
        let days: u16 = read_volatile((RTC_BASE + 0x10) as *const u16);
        println!("days:{}", days);
        // println!("times: {}", );
        let hhmmss = read_volatile((RTC_BASE + 0x14) as *const u32);
        let hh = (hhmmss >> 16) & 0x1f;
        let mm = (hhmmss >> 8) & 0x3f;
        let ss = (hhmmss >> 0) & 0x3f;
        
        println!("{}:{}:{}", hh, mm, ss);
    }
}

pub fn init() {
    print_timestamp();
}

