mod packed_bit_struct;
pub use packed_bit_struct::PackedBits;

impl PackedBits
{
    pub fn new() -> Self
    {
        PackedBits {
            trailing_zeros: 0,
            bytes: vec![],
        }
    }
    pub fn from_bytes_and_tz(bytes: Vec<u8>, trailing_zeros: usize) -> Self
    {
        PackedBits {
            trailing_zeros,
            bytes,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self
    {
        PackedBits {
            trailing_zeros: 0,
            bytes,
        }
    }

    pub fn trailing_zeros(&self) -> usize { self.trailing_zeros }

    pub fn bytes(&self) -> &[u8] { &self.bytes[..] }

    pub fn add_byte(&mut self, byte: u8) { self.bytes.push(byte); }

    pub fn append(&mut self, bytes: &mut Vec<u8>, trailing_zeros: usize)
    {
        self.bytes.append(bytes);
    }
}
