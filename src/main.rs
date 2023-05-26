#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(asm_const)]
#![feature(naked_functions)]

use core::arch::asm;
use core::fmt::Write;

use aarch64_cpu::registers::CurrentEL;
use tock_registers::interfaces::Readable; // <-- Trait needed to use `write()` and `set()`.
use fdt::Fdt;

use crate::uart::UART;

#[macro_use]
extern crate log;

mod boot;
mod kalloc;
mod logging;
mod panic;
mod rtc;
mod trap;
pub mod uart;

use trapframe::TrapFrame;

#[no_mangle]	// export a function 'trap_handler'
extern "C" fn trap_handler(tf: &mut TrapFrame) {
    match tf.trap_num {
        0x3 => {
            info!("TRAP: Breakpoint");
        }
        _ => panic!("TRAP: {:#x?}", tf),
    }
}

#[no_mangle]
extern "C" fn main(device_tree: usize) {
    // kalloc::init();
    uart::init();
    unsafe {
        trapframe::init();
    }
    // logging::init();
    // loop {
    //     // info!("debug 000");
    //     write!(UART.lock(), "debug 000");
    //     for _i in 0..2 {
    //         unsafe {
    //             asm!("wfe");
    //         }
    //     }
    // }
    println!("User Hello World!");
    println!("test for main");

    rtc::init();

    unsafe {
        let mut sp: usize;
        asm!("ldr	{tmp}, ={boot_stack}", tmp = out(reg) sp, boot_stack = sym boot::STACK);
        println!("kalloc initialized: {:#x}", sp);
        asm!("mov {tmp}, sp", tmp = out(reg) sp);
        println!("kalloc initialized: {:#x}", sp);
        println!("sp addr: {:#x}", boot::STACK.as_ptr() as usize + boot::STACK_SIZE);

        let tmp: usize;
        asm!(
            "mrs {tmp}, CurrentEL",
            tmp = out(reg) tmp
        );
        println!("currentel: {}", tmp >> 2);
    }

    // info!("version: {:#x}", ar);
    println!("device_tree: {:#x}", device_tree);
    panic!("end");

    // println!("debug currentEL: {:?}", CurrentEL.read(CurrentEL::EL));

    println!("Hello World!");

    println!("debug 0");

    let fdt_ptr = include_bytes!("../boot/dtb/allwinner/sun50i-h616-orangepi-zero2.dtb");
    println!("debug 0");
    // let fdt = unsafe {
    //     Fdt::from_ptr(fdt_ptr.as_ptr()).unwrap()
    // };
    println!("debug 1");
    
    // let fdt =
        // unsafe { Fdt::from_ptr(0x000000007bf3db20 as *const u8).unwrap() };
        // unsafe { Fdt::from_ptr(0x4FA00000 as *const u8).unwrap() };
    // println!("There has {} CPU(s)", fdt.cpus().count());

    // fdt.memory().regions().for_each(|x| {
    //     println!(
    //         "memory region {:#X} - {:#X}",
    //         x.starting_address as usize,
    //         x.starting_address as usize + x.size.unwrap()
    //     );
    // });

    // println!("debug 1:{:? }", fdt.chosen());
    // println!("debug 2:{:? }", fdt.strings());

    // let node = fdt.all_nodes();

    // for child in node {
    //     if let Some(compatible) = child.compatible() {
    //         println!("    {}", child.name);
    //     }
    // }
}
