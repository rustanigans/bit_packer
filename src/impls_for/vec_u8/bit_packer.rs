use crate::{BitPacker, PackedBits};

impl BitPacker for Vec<u8>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { bits.append(self, 0); }
}
