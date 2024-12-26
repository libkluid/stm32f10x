#![no_std]
#![no_main]

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = _start;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    stm32f10x::reset();
    setup()
}

unsafe fn setup() -> ! {
    stm32f10x::clock::enable_lse();
    stm32f10x::debug::enable_mco();

    let rcc = stm32f10x::peripherals::Rcc::get();

    // init led
    rcc.apb2_enr.mask_word(stm32f10x::mask::Or(0x0000_0004));

    let iop = stm32f10x::peripherals::Gpio::iopa();
    iop.crl.mask_word(stm32f10x::mask::And(0xFF0F_FFFF));
    iop.crl.mask_word(stm32f10x::mask::Or(0x0030_0000));

    loop {
        iop.odr.mask_word(stm32f10x::mask::Or(0x0000_0020));
        stm32f10x::clock::delay_s(2);

        iop.odr.mask_word(stm32f10x::mask::And(!0x0000_0020));
        stm32f10x::clock::delay_s(2);
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
