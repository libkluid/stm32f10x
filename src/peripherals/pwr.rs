use crate::Register;

pub struct Pwr {
    pub cr: Register,
    pub csr: Register,
}

impl Pwr {
    const PTR: *const Pwr = 0x4000_7000 as *const Pwr;

    pub const unsafe fn get() -> &'static Pwr {
        &*Self::PTR
    }
}
