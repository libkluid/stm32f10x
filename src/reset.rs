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

    // Wait until HSI is ready
    while rcc.cr.read_word().bit_of(1) == false {
        crate::asm::nop();
    }

    // MCO, Prescaler off
    // HSI as system clock
    rcc.cfgr.mask_word(mask::And(0xF8FF_0000));
    // PLL off & CSS off & HSE off
    rcc.cr.mask_word(mask::And(0xFEF6_FFFF));
    // HSEBYP off
    rcc.cr.mask_word(mask::And(0xFFFB_FFFF));
    // PLL off
    rcc.cfgr.mask_word(mask::And(0xFF80_FFFF));
    // Creat Flags and disable all clock interrupts
    rcc.cir.write_word(0x009F_0000);
}

unsafe fn advance_to_external_oscillator() {
    let rcc = peripherals::Rcc::get();

    // HSEBYP = 0
    // HSEBYP bit can be written only when HSEON is 0
    rcc.cfgr.mask_word(mask::And(0xFFFB_FFFF));

    // HSEON = 1
    rcc.cr.mask_word(mask::Or(0x0001_0000));

    // Wait until HSE is ready
    while rcc.cr.read_word().bit_of(17) == false {
        crate::asm::nop();
    }

    // HSE as system clock
    rcc.cfgr.mask_word(mask::And(0xFFFF_FFFC));
    rcc.cfgr.mask_word(mask::Or(0x0000_0001));

    // Wait until HSE is system clock
    while rcc.cfgr.read_word().bit_range(2..4) != 0b01 {
        crate::asm::nop();
    }

    // HSI off
    rcc.cr.mask_word(mask::And(0xFFFFFFFE));

    // Wait until HSI is off
    while rcc.cr.read_word().bit_of(1) == true {
        crate::asm::nop();
    }
}

// pll assumes HSE is available and it's frequency is 8MHz
unsafe fn enable_pll() {
    let rcc = peripherals::Rcc::get();

    // config prescalers
    // - reset all prescalers
    rcc.cfgr.mask_word(mask::And(0xFF00_000F));
    // - USB prescaler = 1.5
    rcc.cfgr.mask_word(mask::Or(0x0040_0000));
    // - PLLXTPRE = HSE/1
    rcc.cfgr.mask_word(mask::Or(0x0000_0000));
    // - ADC prescaler = SYSCLK/6
    rcc.cfgr.mask_word(mask::Or(0x0000_8000));
    // APB2 prescaler = SYSCLK/1
    rcc.cfgr.mask_word(mask::Or(0x0000_0000));
    // APB1 prescaler = SYSCLK/2
    rcc.cfgr.mask_word(mask::Or(0x0000_0400));
    // AHB prescaler = SYSCLK/1
    rcc.cfgr.mask_word(mask::Or(0x0000_0000));

    // PLLSRC = 1 (HSE)
    rcc.cfgr.mask_word(mask::Or(0x0001_0000));

    // PLLMUL = 7 (x9 = 72MHz)
    rcc.cfgr.mask_word(mask::And(0xFFC3_FFFF));
    rcc.cfgr.mask_word(mask::Or(0b0111 << 18));

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
}

pub unsafe fn reset() {
    copy_sections();
    initialize_rc_oscillator();
    // TODO: enable only if HSE is available
    if false {
        advance_to_external_oscillator();
        enable_pll();
    }
}
