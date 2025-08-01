use log::error;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{
    display,
    framebuffer::DISPLAY,
    gdt, print, println,
    utils::{Global, hlt_loop},
};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
static PICS: Global<ChainedPics> = Global::uninit();
static IDT: Global<InterruptDescriptorTable> = Global::uninit();
static KEYBOARD: Global<Keyboard<layouts::Us104Key, ScancodeSet1>> = Global::uninit();
static TIMER: Global<Timer> = Global::uninit();

pub fn init_idt() {
    gdt::init_tss();
    KEYBOARD.set(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    ));
    TIMER.set(Timer { ticks: 0 });

    unsafe { PICS.set(ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)) };
    let pics = PICS.get().expect("PICS uninitialized");
    unsafe { pics.initialize() };
    x86_64::instructions::interrupts::enable();

    IDT.set(InterruptDescriptorTable::new());
    let idt = IDT.get().expect("IDT uninitialized");

    unsafe {
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        idt[InterruptIndex::Timer as u8].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
    };

    idt.load();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl From<InterruptIndex> for u8 {
    fn from(index: InterruptIndex) -> Self {
        index as u8
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let timer = TIMER.get().expect("Timer uninitialized");
    timer.ticks += 1;
    if timer.ticks % 6 == 0 {
        display!().blink_caret();
    }

    notify_end_of_interrupt(InterruptIndex::Keyboard);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let keyboard = KEYBOARD.get().expect("Keyboard uninitialized");

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode)
        && let Some(key) = keyboard.process_keyevent(key_event)
    {
        match key {
            DecodedKey::Unicode(character) => print!("{}", character),
            DecodedKey::RawKey(key) => print!("raw {:?}", key),
        }
        display!().blink_caret();
    }

    notify_end_of_interrupt(InterruptIndex::Keyboard);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    error!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

fn notify_end_of_interrupt(interrupt_index: InterruptIndex) {
    unsafe {
        PICS.get()
            .expect("PICS uninitialized")
            .notify_end_of_interrupt(interrupt_index.into());
    }
}

struct Timer {
    ticks: u64,
}
