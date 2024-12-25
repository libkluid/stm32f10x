use crate::{mask, peripherals};

extern "C" {
    // load address(LMA) of .data section
    static _sidata: u32;

    // start address of .data section
    static _sdata: u32;
    // end address of .data section
    static _edata: u32;

    // start address of .bss section
    static _sbss: u32;
    // end address of .bss section
    static _ebss: u32;
}

unsafe fn copy_sections() {
    let sidata = &_sidata as *const u32;
    let sdata = &_sdata as *const u32 as usize;
    let edata = &_edata as *const u32 as usize;
    let count = (edata - sdata) / core::mem::size_of::<u32>();
    core::ptr::copy_nonoverlapping::<u32>(sidata, sdata as *mut u32, count);

    let sbss = &_sbss as *const u32 as usize;
    let ebss = &_ebss as *const u32 as usize;
    let count = (ebss - sbss) / core::mem::size_of::<u32>();
    core::ptr::write_bytes(sbss as *mut u32, 0, count);
}

unsafe fn initialize_rc_oscillator() {
    let rcc = peripherals::Rcc::get();

    // init clocks
    rcc.cr.write_word(0x0000_0001);
    rcc.cfgr.mask_word(mask::And(0xF0FF_0000));
    rcc.cr.mask_word(mask::And(0xFEF6_FFFF));
    rcc.cr.mask_word(mask::And(0xFFFB_FFFF));
    rcc.cfgr.mask_word(mask::And(0xFF80_FFFF));
    rcc.cir.write_word(0x009F_0000);
}

pub unsafe fn reset() {
    copy_sections();
    initialize_rc_oscillator();
}
