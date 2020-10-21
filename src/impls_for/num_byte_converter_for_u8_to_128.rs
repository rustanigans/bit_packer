use crate::NumByteConverter;

macro_rules! impl_for
{
    ($($t:ty)*) =>
    {
        $(
            impl NumByteConverter<$t> for $t
            {
                fn into_be_byte_vec(self) -> Vec<u8>
                {
                    self.to_be_bytes().to_vec()
                }

                unsafe fn from_be_byte_vec(bytes: Vec<u8>) -> Self
                {
                    debug_assert!(<$t>::BYTE_SIZE <= bytes.len());

                    let bytes = *(bytes[..<$t>::BYTE_SIZE].as_ptr() as *const [u8; <$t>::BYTE_SIZE])
                        as [u8; <$t>::BYTE_SIZE];
                    <$t>::from_be_bytes(bytes)
                }
            }

        )*
    }
}

impl_for!(u8 u16 u32 u64 u128);
