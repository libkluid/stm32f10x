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

unsafe fn init_clock() {
    let rcc = peripherals::Rcc::get();

    // init clocks
    rcc.cfgr.write_word(0x0000_0000);
    rcc.cr.write_word(0x0000_0001);

    // Wait until HSI is ready
    while rcc.cr.read_word().bit_of(1) == false {
        crate::asm::nop();
    }

    // Creat Flags and disable all clock interrupts
    rcc.cir.write_word(0x009F_0000);
}

unsafe fn use_hse_pll() {
    let rcc = peripherals::Rcc::get();

    // HSEON = 1
    rcc.cr.mask_word(mask::Or(0x0001_0000));

    // Wait until HSE is ready
    while rcc.cr.read_word().bit_of(17) == false {
        crate::asm::nop();
    }

    // config prescalers
    // - reset all prescalers
    rcc.cfgr.write_word(0x001D_8400);

    // PLLON = 1
    rcc.cr.mask_word(mask::Or(0x0100_0000));

    // Wait until PLL is ready
    while rcc.cr.read_word().bit_of(25) == false {
        crate::asm::nop();
    }

    // PLL as system clock
    rcc.cfgr.mask_word(mask::And(0xFFFF_FFFC));
    rcc.cfgr.mask_word(mask::Or(0x0000_0002));

    // Wait until PLL is system clock
    while rcc.cfgr.read_word().bit_range(2..4) != 0b10 {
        crate::asm::nop();
    }

    // HSI off
    rcc.cr.mask_word(mask::And(0xFFFFFFFE));

    // Wait until HSI is off
    while rcc.cr.read_word().bit_of(1) == true {
        crate::asm::nop();
    }
}

pub unsafe fn reset() {
    copy_sections();
    init_clock();
    use_hse_pll()
}
