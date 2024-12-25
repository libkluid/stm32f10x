use core::cell::UnsafeCell;

#[repr(transparent)]
pub struct Word {
    pub value: u32,
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
