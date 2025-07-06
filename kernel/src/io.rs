use core::fmt;

use x86_64::instructions::interrupts;

use crate::{
    framebuffer::{DISPLAY, init_display},
    serial::{SERIAL1, init_serial},
};

pub fn init(framebuffer: &'static mut bootloader_api::info::FrameBuffer) {
    init_display(framebuffer);
    init_serial(0x3F8);
    DISPLAY.get().expect("display uninit").clear();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        use core::fmt::Write;
        SERIAL1
            .get()
            .expect("serial uninit")
            .write_fmt(args)
            .expect("Printing to serial failed");

        DISPLAY
            .get()
            .expect("display uninit")
            .write_fmt(args)
            .expect("Printing to display failed");
    });
}
