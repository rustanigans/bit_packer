use crate::{PackedBits, Push};

pub trait BitPacker<U>{
    fn add_to_packed_bits(self, bits: &mut U);

}

impl<T,U:Push<T>> BitPacker<U> for T
{
    fn add_to_packed_bits(self, bits: &mut U) {
        bits.push(self)
    }
}