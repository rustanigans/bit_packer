use crate::{traits::shift::Shift, BitPacker, PackedBits};

impl Shift for PackedBits
{
    fn shift(&mut self, mut val: Box<dyn BitPacker>)
    {
        val.extract_from_packed_bits(self);
    }
}
