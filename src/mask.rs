use crate::memory::Word;

pub trait BitMask {
    fn apply(&self, value: Word) -> Word;
}

#[repr(transparent)]
pub struct And(pub u32);
#[repr(transparent)]
pub struct Or(pub u32);
#[repr(transparent)]
pub struct Xor(pub u32);

impl BitMask for And {
    #[inline]
    fn apply(&self, word: Word) -> Word {
        Word {
            value: word.value & self.0,
        }
    }
}

impl BitMask for Or {
    #[inline]
    fn apply(&self, value: Word) -> Word {
        Word {
            value: value.value | self.0,
        }
    }
}

impl BitMask for Xor {
    #[inline]
    fn apply(&self, word: Word) -> Word {
        Word {
            value: word.value ^ self.0,
        }
    }
}
