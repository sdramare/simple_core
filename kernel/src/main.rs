#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(negative_impls)]

bootloader_api::entry_point!(kernel_main);

use core::panic::PanicInfo;

use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};

use crate::utils::hlt_loop;

mod framebuffer;
mod gdt;
mod interrupts;
mod io;
mod logger;
mod serial;
mod utils;

// ↓ this replaces the `_start` function ↓
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        io::init(framebuffer);
        logger::init();
        interrupts::init_idt();

        println!("{} {}", "Hello", "World!");

        log::error!("This is an error message");
        log::warn!("This is a warning message");
        log::info!("This is an info message");

        x86_64::instructions::interrupts::int3();

        println!("It did not crash!");
        log::info!("Kernel initialized successfully.");
        clear!();

        let ptr = 0xdeadbeaf as *mut u8;
        unsafe {
            *ptr = 42;
        }

        println_color!(Rgb888::GREEN, "Welcome to the Simple Core Kernel!");
        print_color!(Rgb888::GREEN, ">");
    }

    hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // This function is called when a panic occurs.
    // In a no_std environment, we typically enter an infinite loop.
    log::error!("\nKernel panic: {_info}");
    hlt_loop();
}
