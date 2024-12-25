use crate::mask::BitMask;
use crate::memory::{VolatileCell, Word};

pub struct Register {
    base: VolatileCell<Word>,
}

impl Register {
    #[inline]
    pub unsafe fn read_word(&self) -> Word {
        self.base.read()
    }

    #[inline]
    pub unsafe fn write_word<W>(&self, word: W)
    where
        W: Into<Word>,
    {
        self.base.write(word.into());
    }

    #[inline]
    pub unsafe fn mask_word<Mask: BitMask>(&self, mask: Mask) {
        let word = self.read_word();
        self.write_word(mask.apply(word));
    }
}
