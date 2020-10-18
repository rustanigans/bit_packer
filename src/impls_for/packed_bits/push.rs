use crate::{Push, PackedBits, BitPacker};

impl<T> Push<T> for PackedBits
    where T: BitPacker
{
    fn push(&mut self, val: T) {
        val.add_to_packed_bits(self)
    }
}
