pub struct PackedBits
{
    pub(super) trailing_zeros: usize,
    pub(super) bytes: Vec<u8>,
}
