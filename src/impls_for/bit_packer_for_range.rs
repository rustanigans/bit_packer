use crate::{BitPacker, PackedBits, PackedRange};

impl<'a, T: Copy> BitPacker for PackedRange<'a, T>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) {
        self.1.pack(self.0, bits);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
        self.1.restore(&mut self.0, bits);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RangeDefinition, RangeDef, Push, Shift};

    #[test]
    fn pack_u8_range()
    {
        let mut pb = PackedBits::default();
        let range:PackedRange<u8> = (5, &RangeDefinition::new(0,7));
        pb.push(&range);

        assert_eq!(&[5u8<<5], pb.bytes());
    }
    
    #[test]
    fn unpack_u8_range() {
        let mut pb = PackedBits::from_bytes(vec![5<<5]);
        let mut range:PackedRange<u8> = (0, &RangeDefinition::new(0,7));
        pb.shift(&mut range);

        assert_eq!(5, range.0);
    }

    #[test]
    fn pack_u32_range()
    {
        let mut pb = PackedBits::default();
        let range:PackedRange<u32> = (5, &RangeDefinition::new(0,7));
        pb.push(&range);

        assert_eq!(&[5u8<<5], pb.bytes());
    }

    #[test]
    fn unpack_u32_range() {
        let mut pb = PackedBits::from_bytes(vec![5<<5]);
        let mut range:PackedRange<u32> = (0, &RangeDefinition::new(0,7));
        pb.shift(&mut range);

        assert_eq!(5, range.0);
    }
}