use lazy_static::lazy_static;
use x86_64::{
    VirtAddr,
    instructions::tables::load_tss,
    registers::segmentation::{CS, Segment},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // This can be any other IST entry too not just 0.

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE]; // Must be "static mut"and not just "static", otherwise it will be mapped to a read-only page by the bootloader.

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end // stack_end is returned because stack grows downward in x86
        };

        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    unsafe {
        // We changed our GDT, so we should reload cs, the code segment register. This is required since the old segment selector could now point to a different GDT descriptor (e.g., a TSS descriptor).
        CS::set_reg(GDT.1.code_selector);

        // We loaded a GDT that contains a TSS selector, but we still need to tell the CPU that it should use that TSS
        load_tss(GDT.1.tss_selector);
    }
}
