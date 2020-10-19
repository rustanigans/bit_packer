use crate::{BitPacker, PackedBits};

// TODO Tests required

impl BitPacker for u8
{
    fn add_to_packed_bits(self, bits: &mut PackedBits) { bits.add_byte(self); }
}

macro_rules! impl_for{
($($i:ident:$n:expr)*) => {
        $(
            impl BitPacker for $i {
                fn add_to_packed_bits(self, bits: &mut PackedBits) {
                    let mut be_bytes = self.to_be_bytes();
                    &mut unsafe {
                        Vec::from_raw_parts(
                            be_bytes.as_mut_ptr(), $n, $n)
                    }.add_to_packed_bits(bits);
                }
            }
        )*
    }
}

impl_for!(u16:2 u32:4 u64:8 u128:16);
