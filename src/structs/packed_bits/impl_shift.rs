use crate::{traits::shift::Shift, BitPacker, PackedBits};

impl Shift for PackedBits
{
    fn shift(&mut self, val: &mut dyn BitPacker)
    {
        val.extract_from_packed_bits(self);
    }
}
