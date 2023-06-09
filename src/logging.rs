use core::fmt::{self, Write, Arguments};

use log::{self, info, Level, LevelFilter, Log, Metadata, Record};

use crate::uart::UART;

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        print_in_color(
            format_args!("[{}] {}", record.level(), record.args()),
            level_to_color_code(record.level()),
        );
    }
    fn flush(&self) {}
}

impl Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = UART.lock().write_str(s);
        Ok(())
    }
}

pub fn init() {
    log::set_logger(&Logger).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
    info!("logging module initialized");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        // $crate::logging::Logger.write_fmt(format_args!($($arg)*));
        $crate::logging::print_args(format_args!($($arg)*))
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m\r\n", $color_code as u8, $args)
    }};
}

pub fn print_args(args: fmt::Arguments) {
    Logger.write_fmt(args).expect("cant' write arguments")
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    Logger
        .write_fmt(with_color!(args, color_code))
        .expect("can't write color string in logging module.");
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
