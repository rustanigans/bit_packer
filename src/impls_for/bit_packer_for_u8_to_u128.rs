use crate::{BitPacker, PackedBits, NumByteConverter};

impl BitPacker for u8
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { bits.add_byte(*self); }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) { *self = bits.take_byte() }
}

macro_rules! impl_for{
($($i:ident:$n:expr)*) => {
        $(
            impl BitPacker for $i {
                fn add_to_packed_bits(&self, bits: &mut PackedBits) {
                    bits.append(&mut self.into_be_byte_vec(),0);
                }

                fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
                {
                    const BIT_SIZE: usize = std::mem::size_of::<$i>() * 8;
                    const BYTE_SIZE: usize = std::mem::size_of::<$i>();

                    unsafe {
                        let bytes = bits.take_bits(BIT_SIZE);
                        debug_assert_eq!(BYTE_SIZE, bytes.len());
                        let bytes = *(bytes[0..BYTE_SIZE].as_ptr() as *const [u8; BYTE_SIZE]) as [u8; BYTE_SIZE];
                        *self = $i::from_be_bytes(bytes);
                    }
                }
            }
        )*
    }
}
impl_for!(u16:2 u32:4 u64:8 u128:16);
