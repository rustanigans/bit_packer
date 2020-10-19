use crate::{PackedBits};

pub trait BitPacker {
    fn add_to_packed_bits(&self, bits: &mut PackedBits);
}
