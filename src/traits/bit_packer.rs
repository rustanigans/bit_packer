use crate::{PackedBits, Push};

pub trait BitPackerInternal<U>
{
    fn add_to_packed_bits(self, bits: &mut U);
}

impl<T, U: Push<T>> BitPackerInternal<U> for T
{
    fn add_to_packed_bits(self, bits: &mut U) { bits.push(self) }
}

pub trait BitPacker: BitPackerInternal<PackedBits>
{
}
