use x86_64::VirtAddr;
use x86_64::instructions::segmentation::{CS, Segment};
use x86_64::instructions::tables::load_tss;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;

use crate::utils::Global;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static TSS: Global<TaskStateSegment> = Global::uninit();
static GDT: Global<GlobalDescriptorTable> = Global::uninit();

pub fn init_tss() {
    TSS.set(TaskStateSegment::new());
    let tss = TSS.get().expect("TSS uninitialized");

    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(&raw const STACK);
        stack_start + STACK_SIZE as u64
    };

    GDT.set(GlobalDescriptorTable::new());
    let gdt = GDT.get().expect("GDT uninitialized");
    let code_selector: x86_64::structures::gdt::SegmentSelector =
        gdt.append(Descriptor::kernel_code_segment());
    gdt.append(Descriptor::kernel_data_segment());
    let tss_selector = gdt.append(Descriptor::tss_segment(tss));
    /* gdt.append(Descriptor::UserSegment(0));
    gdt.append(Descriptor::user_code_segment());
    gdt.append(Descriptor::user_data_segment()); */
    gdt.load();

    unsafe {
        CS::set_reg(code_selector);
        load_tss(tss_selector);
    }
}
