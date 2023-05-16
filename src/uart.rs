use core::fmt::Write;
use ns16550a::*;
use spin::{Mutex, Lazy};

pub static UART: Lazy<Mutex<Uart>> = Lazy::new(|| {
    Mutex::new(Uart::new(0x0500_0000))
});

pub fn init() {
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
    write!(UART.lock(), "Hello, world!\n\r");
}
