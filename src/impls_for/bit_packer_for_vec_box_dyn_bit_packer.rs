use crate::{BitPacker, PackedBits, Push};

impl BitPacker for Vec<Box<dyn BitPacker>>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        for item in self.iter()
        {
            bits.push(item);
        }
    }

    fn extract_from_packed_bits(&mut self, _bits: &mut PackedBits)
    {
        panic!("Cannot extract Vec<Box<dyn BitPacker>> automatically");
    }
}
