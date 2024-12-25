use crate::Register;

pub struct Gpio {
    pub crl: Register,
    pub crh: Register,
    pub idr: Register,
    pub odr: Register,
    pub bsrr: Register,
    pub brr: Register,
    pub lckr: Register,
}

impl Gpio {
    const IOPA: *const Gpio = 0x4001_0800 as *const Gpio;
    const IOPB: *const Gpio = 0x4001_0C00 as *const Gpio;
    const IOPC: *const Gpio = 0x4001_1000 as *const Gpio;
    const IOPD: *const Gpio = 0x4001_1400 as *const Gpio;
    const IOPE: *const Gpio = 0x4001_1800 as *const Gpio;

    pub const unsafe fn iopa() -> &'static Gpio {
        &*Self::IOPA
    }

    pub const unsafe fn iopb() -> &'static Gpio {
        &*Self::IOPB
    }

    pub const unsafe fn iopc() -> &'static Gpio {
        &*Self::IOPC
    }

    pub const unsafe fn iopd() -> &'static Gpio {
        &*Self::IOPD
    }

    pub const unsafe fn iope() -> &'static Gpio {
        &*Self::IOPE
    }
}
