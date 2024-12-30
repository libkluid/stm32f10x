#![no_std]
#![no_main]

use core::fmt::Write;

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = _start;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    stm32f10x::reset();
    setup()
}

unsafe fn init_led() {
    let rcc = stm32f10x::peripherals::Rcc::get();
    rcc.apb2_enr.mask_word(stm32f10x::mask::Or(0x0000_0010));

    let iop = stm32f10x::peripherals::Gpio::iopa();
    iop.crl.mask_word(stm32f10x::mask::And(0xFF0F_FFFF));
    iop.crl.mask_word(stm32f10x::mask::Or(0x0030_0000));
}

unsafe fn led_out(state: bool) {
    let iop = stm32f10x::peripherals::Gpio::iopc();
    if state {
        iop.odr.mask_word(stm32f10x::mask::Or(0x0000_2000));
    } else {
        iop.odr.mask_word(stm32f10x::mask::And(!0x0000_2000));
    }
}

unsafe fn init_usart2() {
    let rcc = stm32f10x::peripherals::Rcc::get();
    rcc.apb1_enr.mask_word(stm32f10x::mask::Or(0x0002_0000));

    let iop = stm32f10x::peripherals::Gpio::iopa();
    iop.crl.mask_word(stm32f10x::mask::And(0xFFFF_00FF));
    iop.crl.mask_word(stm32f10x::mask::Or(0x0000_4B00));

    let usart = stm32f10x::peripherals::Usart::usart2();
    usart.brr.write_word(0x0000_0EA6);
    usart.cr1.write_word(0x0000_0000);
    usart.cr1.mask_word(stm32f10x::mask::Or(0x0000_0008));
    usart.cr1.mask_word(stm32f10x::mask::Or(0x0000_2000));
}

unsafe fn setup() -> ! {
    stm32f10x::clock::enable_lse();
    stm32f10x::debug::enable_mco();

    init_led();
    init_usart2();

    let usart2 = stm32f10x::peripherals::Usart::usart2();
    for i in 0..u32::MAX {
        write!(usart2, "Hello, world! {}\r\n", i).unwrap();
        led_out(false);
        stm32f10x::clock::delay_s(1);

        led_out(true);
        stm32f10x::clock::delay_s(1);
    }

    loop {
        stm32f10x::clock::delay_s(1);
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
