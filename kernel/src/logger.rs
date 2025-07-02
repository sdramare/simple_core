use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};

use crate::{framebuffer::DISPLAY, serial::SERIAL1};

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
            use core::fmt::Write;

            let level = record.level();
            let color = match level {
                log::Level::Error => Rgb888::RED,
                log::Level::Warn => Rgb888::YELLOW,
                log::Level::Info => Rgb888::GREEN,
                log::Level::Debug => Rgb888::CYAN,
                log::Level::Trace => Rgb888::GREEN,
            };

            let message = format_args!("{}: {}\n", record.level(), record.args());
            SERIAL1
                .get()
                .expect("serial uninit")
                .write_fmt(message)
                .expect("Printing to serial failed");

            DISPLAY
                .get()
                .expect("display uninit")
                .color(color)
                .write_fmt(message)
                .expect("Printing to display failed");
        }
    }

    fn flush(&self) {}
}
