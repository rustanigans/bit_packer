use crate::{PackedBits};

pub trait BitPacker {
    fn add_to_packed_bits(self, bits: &mut PackedBits);
}

pub trait PPush<T> {
    fn push(&mut self, val: T) ;
}

impl<T> PPush<T> for PackedBits
    where T: BitPacker
{
    fn push(&mut self, val: T) {
        val.add_to_packed_bits(self)
    }
}
