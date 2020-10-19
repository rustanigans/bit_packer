mod packed_bit_struct;
use crate::utils::{shift_left::shift_left, shift_right::shift_right};
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

    pub fn append(&mut self, bytes: &mut Vec<u8>, mut trailing_zeros: usize)
    {
        let len = self.bytes.len();
        if bytes.len() == 0
        {
            return;
        }
        if len == 0 || self.trailing_zeros == 0
        {
            self.bytes.append(bytes);
            self.trailing_zeros = trailing_zeros;
            return;
        }

        trailing_zeros = shift_right(bytes, 8-self.trailing_zeros, trailing_zeros);

        let (trim, tz) = shift_left(bytes, 8, trailing_zeros);
        self.trailing_zeros = tz;
        unsafe {
            let last = self.bytes.get_unchecked_mut(len - 1);
            *last = *last | trim;
        }
        self.bytes.append(bytes);
    }
}
