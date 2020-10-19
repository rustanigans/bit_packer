use super::PackedBits;

impl From<Vec<u8>> for PackedBits {
    fn from(bytes: Vec<u8>) -> Self {
        PackedBits::from_bytes(bytes)
    }
}