use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct Word {
    pub value: u32,
}

impl Word {
    pub fn bit_of(&self, offset: u32) -> bool {
        assert!(offset < 8 * core::mem::size_of::<u32>() as u32);
        (self.value & (1 << offset)) != 0
    }

    pub fn bit_range(&self, range: core::ops::Range<u32>) -> u32 {
        assert!(range.end < 8 * core::mem::size_of::<u32>() as u32);
        let mask = (1 << (range.end - range.start)) - 1;
        (self.value >> range.start) & mask
    }

    pub fn lower_half(&self) -> u16 {
        (0x0000_FFFF & self.value) as u16
    }

    pub fn upper_half(&self) -> u16 {
        ((0xFFFF_0000 & self.value) >> 16) as u16
    }
}

impl From<u32> for Word {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

pub struct VolatileCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    pub unsafe fn read(&self) -> T {
        core::ptr::read_volatile(self.inner.get())
    }

    pub unsafe fn write(&self, value: T) {
        core::ptr::write_volatile(self.inner.get(), value);
    }
}
