use crate::{traits::shift::Shift, BitPacker, PackedBits};

impl Shift for PackedBits
{
    fn shift<T: BitPacker>(&mut self, val: &mut T)
    {
        val.extract_from_packed_bits(self);
    }
}
