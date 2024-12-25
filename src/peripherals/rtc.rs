use crate::Register;

pub struct Rtc {
    pub crh: Register,
    pub cfl: Register,
    pub prlh: Register,
    pub prll: Register,

    pub divh: Register,
    pub divl: Register,
    pub cnth: Register,
    pub cntl: Register,

    pub alrh: Register,
    pub alrl: Register,
}

impl Rtc {
    const PTR: *const Rtc = 0x4000_2800 as *const Rtc;

    pub const unsafe fn get() -> &'static Rtc {
        &*Self::PTR
    }
}
