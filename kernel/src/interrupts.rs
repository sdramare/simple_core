use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, print, println, utils::Global};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
static PICS: Global<ChainedPics> = Global::uninit();
static IDT: Global<InterruptDescriptorTable> = Global::uninit();

pub fn init_idt() {
    gdt::init_tss();
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
    };

    idt.load();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl Into<u8> for InterruptIndex {
    fn into(self) -> u8 {
        self as u8
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
    print!(".");
    unsafe {
        PICS.get()
            .expect("PICS uninitialized")
            .notify_end_of_interrupt(InterruptIndex::Timer.into());
    }
}
