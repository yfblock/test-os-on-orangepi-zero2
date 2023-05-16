#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(asm_const)]

#[macro_use]
extern crate log;

mod panic;
mod console;
mod boot;
mod logging;
pub mod uart;
mod kalloc;

#[no_mangle]
extern "C" fn main() {
    kalloc::init();
    uart::init();
    logging::init();
    info!("Hello World!");
}