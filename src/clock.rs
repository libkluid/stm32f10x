use crate::{cortex_m, mask, peripherals};

pub unsafe fn enable_lse() {
    let rcc = peripherals::Rcc::get();
    let pwr = peripherals::Pwr::get();

    // set BKPEN, PWREN
    rcc.apb1_enr.mask_word(mask::Or(0x1800_0000));

    // DBP = 1
    pwr.cr.mask_word(mask::Or(0x0000_0100));

    // LSEON = 1
    rcc.bdcr.mask_word(mask::Or(0x0000_0001));

    // Wait until LSE is ready
    while rcc.bdcr.read_word().bit_of(1) == false {
        cortex_m::asm::nop();
    }

    // RTCSEL = 0b01 (LSE)
    rcc.bdcr.mask_word(mask::And(0xFFFF_FCFF));
    rcc.bdcr.mask_word(mask::Or(0x0000_0100));

    // RTCEN = 1
    rcc.bdcr.mask_word(mask::Or(0x0000_8000));

    // DBP = 0
    pwr.cr.mask_word(mask::Or(0x0000_0100));
}

pub unsafe fn rtc_now() -> u32 {
    let rtc = peripherals::Rtc::get();

    let lower = rtc.cntl.read_word().lower_half() as u32;
    let upper = rtc.cnth.read_word().lower_half() as u32;

    (upper << 16) | lower
}

pub unsafe fn delay_s(second: u32) {
    let now = rtc_now();

    while rtc_now() != now + second {
        cortex_m::asm::nop();
    }
}
