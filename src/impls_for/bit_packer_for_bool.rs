use crate::*;

impl BitPacker for bool
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        let mut bit = if *self { vec![1 << 7] } else { vec![0] };
        bits.append(&mut bit, 7)
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        *self = bits.take_bits(1)[0] == 128
    }
}
