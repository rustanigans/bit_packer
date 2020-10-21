use crate::{BitPacker, PackedBits, Push, Shift};

impl<X: BitPacker, Y: BitPacker> BitPacker for (X,Y) {
    fn add_to_packed_bits(&self, bits: &mut PackedBits) {
        bits.push(&self.0);
        bits.push(&self.1);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
        bits.shift(&mut self.0);
        bits.shift(&mut self.1);
    }
}

impl<X: BitPacker, Y: BitPacker, Z: BitPacker> BitPacker for (X,Y,Z) {
    fn add_to_packed_bits(&self, bits: &mut PackedBits) {
        bits.push(&self.0);
        bits.push(&self.1);
        bits.push(&self.2);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
        bits.shift(&mut self.0);
        bits.shift(&mut self.1);
        bits.shift(&mut self.2);
    }
}

impl<X: BitPacker, Y: BitPacker, Z: BitPacker, W: BitPacker> BitPacker for (X,Y,Z,W) {
    fn add_to_packed_bits(&self, bits: &mut PackedBits) {
        bits.push(&self.0);
        bits.push(&self.1);
        bits.push(&self.2);
        bits.push(&self.3);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
        bits.shift(&mut self.0);
        bits.shift(&mut self.1);
        bits.shift(&mut self.2);
        bits.shift(&mut self.3);
    }
}