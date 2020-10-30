use crate::{BitPacker, PackedBits};

impl BitPacker for Box<dyn BitPacker>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        self.as_ref().add_to_packed_bits(bits);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        self.as_mut().extract_from_packed_bits(bits);
    }
}
