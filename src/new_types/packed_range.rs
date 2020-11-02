use crate::{BitPacker, PackedBits, RangeDef};

pub type PackedRange<'a, T> = (T, &'a (dyn RangeDef<T>));

impl<'a, T: Send + Copy> BitPacker for PackedRange<'a, T>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        self.1.pack(self.0, bits);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        self.1.restore(&mut self.0, bits);
    }
}

pub type PackedRangeMut<'a, T> = (&'a mut T, &'a (dyn RangeDef<T>));

impl<'a, T: Send + Copy> BitPacker for PackedRangeMut<'a, T>{

    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        self.1.pack(*self.0, bits);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        self.1.restore(&mut self.0, bits);
    }
}
