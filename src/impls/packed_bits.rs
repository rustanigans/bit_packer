mod packed_bit_struct;
use crate::utils::{shift_left::shift_left, shift_right::shift_right};
pub use packed_bit_struct::PackedBits;
use std::ops::Div;
use std::borrow::BorrowMut;

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

    pub fn append(&mut self, bytes: &Vec<u8>, mut trailing_zeros: usize)
    {
        let bytes = &mut bytes.to_vec();
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

    pub fn take_bits(&mut self, count: usize) -> Vec<u8> {
        let whole_bytes = count / 8;
        let mut bytes = self.bytes.split_off(whole_bytes);
        std::mem::swap(&mut self.bytes, &mut bytes);
        let leftover = count - whole_bytes * 8;
        if leftover == 0
        {
            return bytes;
        }
        let (mut trim, tz) = shift_left(&mut self.bytes, leftover, self.trailing_zeros);
        self.trailing_zeros = tz;
        trim = trim << 8-leftover;
        bytes.push(trim);
        bytes
    }
}
