
pub(crate) trait NumByteConverter<T:Copy> {
    const BIT_SIZE: usize = std::mem::size_of::<T>() * 8;
    const BYTE_SIZE: usize = std::mem::size_of::<T>();
    fn into_be_byte_vec(self) -> Vec<u8>;
    unsafe fn from_be_byte_vec(bytes: Vec<u8>) -> Self;
}