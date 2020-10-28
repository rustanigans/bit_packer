use crate::PackedBits;

pub trait BitPacker: Send
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits);
    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits);

    fn pack(&self) -> PackedBits
    {
        let mut pb = PackedBits::default();
        self.add_to_packed_bits(&mut pb);
        pb
    }

    fn unpack(&mut self, mut bytes: Vec<u8>)
    {
        let mut pb = PackedBits::default();
        pb.append(&mut bytes, 0);
        self.extract_from_packed_bits(&mut pb);
    }
}
