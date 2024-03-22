use core::fmt::Write;

use dw_apb_uart::DW8250;
use ns16550a::*;
use spin::{mutex::Mutex, Lazy};
use uart_16550::MmioSerialPort;

// pub static UART1: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x0500_0000)));
// // pub static UART: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x0500_0000)));
// // pub static UART: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x0500_0000)));

// pub fn uart_init() {
//     let mut uart = unsafe { MmioSerialPort::new(0x0500_0000) };
//     uart.init();
//     uart.send(b'3');
//     UART1.lock().init(
//         WordLength::EIGHT,
//         StopBits::ONE,
//         ParityBit::DISABLE,
//         ParitySelect::EVEN,
//         StickParity::DISABLE,
//         Break::DISABLE,
//         DMAMode::MODE0,
//         Divisor::BAUD115200,
//     );
// }

pub static UART1: Lazy<Mutex<DW8250>> = Lazy::new(|| Mutex::new(DW8250::new(0x0500_0000)));

pub fn uart_init() {
    UART1.lock().init();
}
