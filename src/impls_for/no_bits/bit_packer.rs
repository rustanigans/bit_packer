use crate::{BitPacker, NoBits, PackedBits};

impl BitPacker for NoBits
{
    fn add_to_packed_bits(self, _bits: &mut PackedBits) {}
}
