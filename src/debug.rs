use crate::{mask, peripherals};

// TODO: Check if PA8 is in use
pub unsafe fn enable_mco() {
    let rcc = peripherals::Rcc::get();

    // Enable PA8 to MCO
    rcc.apb2_enr.mask_word(crate::mask::Or(0x0000_0004));

    // enable IO port A
    let iopa = peripherals::Gpio::iopa();
    iopa.crh.mask_word(mask::And(0xFFFF_FFF0));
    iopa.crh.mask_word(mask::Or(0x0000_000B));

    // MCO using HSE
    rcc.cfgr.mask_word(mask::And(0xF8FF_FFFF));
    rcc.cfgr.mask_word(mask::Or(0x0700_0000));
}
