use crate::Register;

pub struct Rcc {
    pub cr: Register,
    pub cfgr: Register,
    pub cir: Register,
    pub apb2_rstr: Register,

    pub apb1_rstr: Register,
    pub ahb_enr: Register,
    pub apb2_enr: Register,
    pub apb1_enr: Register,

    pub bdcr: Register,
    pub csr: Register,
}

impl Rcc {
    const PTR: *const Rcc = 0x4002_1000 as *const Rcc;

    pub const unsafe fn get() -> &'static Rcc {
        &*Self::PTR
    }
}
