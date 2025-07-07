use core::fmt;

use embedded_graphics::pixelcolor::Rgb888;
use x86_64::instructions::interrupts;

use crate::{
    framebuffer::{ColoredDisplay, DISPLAY, init_display},
    serial::{SERIAL1, init_serial},
};

pub fn init(framebuffer: &'static mut bootloader_api::info::FrameBuffer) {
    init_display(framebuffer);
    init_serial(0x3F8);
    DISPLAY.lock().get().expect("display uninit").clear();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! clear {
    ($($arg:tt)*) => {
        $crate::io::_clear()
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_color {
    ($color:expr, $($arg:tt)*) => ($crate::io::_print_color($color, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_color {
    () => ($crate::print!("\n"));
    ($color:expr, $($arg:tt)*) => ($crate::print_color!($color, "{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        use core::fmt::Write;

        SERIAL1
            .lock()
            .get()
            .expect("serial uninit")
            .write_fmt(args)
            .expect("Printing to serial failed");

        DISPLAY
            .lock()
            .get()
            .expect("display uninit")
            .write_fmt(args)
            .expect("Printing to display failed");
    });
}

#[doc(hidden)]
pub fn _print_color(color: embedded_graphics::pixelcolor::Rgb888, args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        use core::fmt::Write;

        SERIAL1
            .lock()
            .get()
            .expect("serial uninit")
            .write_fmt(args)
            .expect("Printing to serial failed");

        ColoredDisplay::new(color)
            .write_fmt(args)
            .expect("Printing to display failed");
    });
}

#[doc(hidden)]
pub fn _clear() {
    interrupts::without_interrupts(|| {
        use core::fmt::Write;

        SERIAL1
            .lock()
            .get()
            .expect("serial uninit")
            .write_fmt(format_args!("---clear---\n"))
            .expect("Printing to serial failed");

        DISPLAY.lock().get().expect("display uninit").clear();
    });
}
