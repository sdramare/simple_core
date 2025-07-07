use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};

use crate::println_color;

const LOGGER: Logger = Logger {};

pub fn init() {
    // Initialize the logger
    log::set_logger(&Logger).expect("Failed to set logger");
    log::set_max_level(log::LevelFilter::Info);
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let color = match level {
                log::Level::Error => Rgb888::RED,
                log::Level::Warn => Rgb888::YELLOW,
                log::Level::Info => Rgb888::GREEN,
                log::Level::Debug => Rgb888::CYAN,
                log::Level::Trace => Rgb888::GREEN,
            };

            println_color!(color, "{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
