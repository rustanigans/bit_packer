mod packed_bit_struct;
pub use packed_bit_struct::PackedBits;

impl PackedBits {
    pub fn add_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) {
        self.bytes.append(bytes);
    }
}