// Copyright (c) 2024 Logan Dhillon. This software is subject to the Bad Technologies Open Software License.

use lazy_static::lazy_static;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{structures::gdt::SegmentSelector, VirtAddr};

use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { core::ptr::addr_of!(STACK) });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_sel = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_sel = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_sel, tss_sel })
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_sel);
        load_tss(GDT.1.tss_sel);
    }
}

struct Selectors {
    code_sel: SegmentSelector,
    tss_sel: SegmentSelector,
}
