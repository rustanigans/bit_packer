use crate::PackedBits;

// TODO Move this
impl From<(Vec<u8>, usize)> for PackedBits
{
    fn from((bytes, tz): (Vec<u8>, usize)) -> Self { PackedBits::from_bytes_and_tz(bytes, tz) }
}
