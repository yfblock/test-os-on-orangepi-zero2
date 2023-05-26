use core::panic::PanicInfo;

use crate::{println, print};

// 程序遇到错误
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    // backtrace();
    // println!("!TEST FINISH!");
    // // loop {}
    // shutdown()
    loop {}
}
