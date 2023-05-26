use core::fmt::Write;

use ns16550a::*;
use spin::{Lazy, Mutex};
use uart_16550::MmioSerialPort;

pub static UART: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x0500_0000)));
// pub static UART: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x0500_0000)));
// pub static UART: Lazy<Mutex<Uart>> = Lazy::new(|| Mutex::new(Uart::new(0x0500_0000)));

pub fn init() {
    let mut uart = unsafe { MmioSerialPort::new(0x0500_0000) };
    uart.init();
    uart.send(b'3');
    UART.lock().init(
        WordLength::EIGHT,
        StopBits::ONE,
        ParityBit::DISABLE,
        ParitySelect::EVEN,
        StickParity::DISABLE,
        Break::DISABLE,
        DMAMode::MODE0,
        Divisor::BAUD115200,
    );
}
