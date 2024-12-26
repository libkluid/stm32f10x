use crate::Register;

pub struct Usart {
    pub sr: Register,
    pub dr: Register,
    pub brr: Register,
    pub cr1: Register,
    pub cr2: Register,
    pub cr3: Register,
    pub gtpr: Register,
}

impl Usart {
    const PTR_USART1: *const Usart = 0x4001_3800 as *const Usart;
    const PTR_USART2: *const Usart = 0x4000_4400 as *const Usart;
    const PTR_USART3: *const Usart = 0x4000_4800 as *const Usart;

    pub const unsafe fn usart1() -> &'static mut Usart {
        &mut *(Self::PTR_USART1 as *mut Usart)
    }

    pub const unsafe fn usart2() -> &'static mut Usart {
        &mut *(Self::PTR_USART2 as *mut Usart)
    }

    pub const unsafe fn usart3() -> &'static mut Usart {
        &mut *(Self::PTR_USART3 as *mut Usart)
    }
}

impl core::fmt::Write for Usart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                while self.sr.read_word().bit_of(7) == false {
                    crate::asm::nop();
                }
                self.dr.write_word(byte as u32);
            }
        }
        Ok(())
    }
}
