use crate::{BitPacker, NoBits, PackedBits};

impl BitPacker for NoBits
{
    fn add_to_packed_bits(&self, _bits: &mut PackedBits) {}

    fn extract_from_packed_bits(&mut self, _bits: &mut PackedBits) {}
}
