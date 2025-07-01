#![no_std]
#![no_main]

bootloader_api::entry_point!(kernel_main);

use bootloader_api::info::FrameBufferInfo;
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;
use core::{cell::UnsafeCell, fmt, panic::PanicInfo};

mod framebuffer;

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
static DISPLAY: Global<framebuffer::Display> = Global::uninit();

static HELLO: &str = "Hello World!";

// ↓ this replaces the `_start` function ↓
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        // extract the framebuffer info and, to satisfy the borrow checker, clone it
        let frame_buffer_info = framebuffer.info().clone();

        // get the framebuffer's mutable raw byte slice
        let raw_frame_buffer = framebuffer.buffer_mut();

        let raw_frame_buffer = as_mut_copy(raw_frame_buffer);

        init_logger(raw_frame_buffer, frame_buffer_info);

        log::error!("This is an error message");
        log::warn!("This is a warning message");
        log::info!("This is an info message");

        init_display(framebuffer);

        DISPLAY.get().unwrap().clear();

        for x in 0..100 {
            println!("{}: {}", x, HELLO);
        }
    }
    loop {}
}

fn init_display(framebuffer: &'static mut bootloader_api::info::FrameBuffer) {
    DISPLAY.set(framebuffer::Display::new(framebuffer));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    DISPLAY.get().unwrap().write_fmt(args).unwrap();
}

fn as_mut_copy<'a>(buffer: &'a mut [u8]) -> &'static mut [u8] {
    // SAFETY: This is safe because we are guaranteed that the buffer is mutable and
    // we are not violating any aliasing rules.
    unsafe { &mut *(buffer as *mut [u8]) }
}

fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));

    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // This function is called when a panic occurs.
    // In a no_std environment, we typically enter an infinite loop.
    loop {}
}

struct Global<T> {
    value: UnsafeCell<Option<T>>,
}

impl<T> Global<T> {
    const fn uninit() -> Self {
        Global {
            value: UnsafeCell::new(Option::None),
        }
    }

    /// Replaces the value, returning the old without dropping either.
    fn set(&self, value: T) -> Option<T> {
        unsafe { self.value.get().replace(Some(value)) }
    }

    fn get(&self) -> Option<&mut T> {
        unsafe { self.value.get().as_mut().and_then(|value| value.as_mut()) }
    }
}

unsafe impl<T> Send for Global<T> where T: Send {}
unsafe impl<T> Sync for Global<T> where T: Send + Sync {}
