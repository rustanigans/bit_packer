use crate::{Push, PackedBits, BitPacker};

impl<T> Push<&T> for PackedBits
    where T: BitPacker
{
    fn push(&mut self, val: &T) {
        val.add_to_packed_bits(self);
    }
}

impl Push<&Box<dyn BitPacker>> for PackedBits {
    fn push(&mut self, val: &Box<dyn BitPacker>) {
        val.add_to_packed_bits(self);
    }
}
