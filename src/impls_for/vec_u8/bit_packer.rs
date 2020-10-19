use crate::{BitPacker, PackedBits};

impl BitPacker for Vec<u8>
{
    /// Copies vec then merges into packed bits
    /// To merge without copy use PackedBits.append directly
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { bits.append(&mut self.to_vec(), 0); }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
        *self = bits.bytes().to_vec();
    }
}
