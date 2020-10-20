use crate::{unions::UnsignedSigned, BitPacker, PackedBits, Push};

impl BitPacker for i8
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        let val = UnsignedSigned::<i8, u8> { unsigned: *self };
        unsafe {
            bits.push(&val.signed);
        }
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
        let val = UnsignedSigned::<i8, u8> { signed: bits.take_byte() };
        unsafe {
            *self = val.unsigned;
        }
    }
}

macro_rules! impl_for {
($($i:ident:$u:ident)*) =>
    {
        $(
            impl BitPacker for $i
            {
                fn add_to_packed_bits(&self, bits: &mut PackedBits)
                {
                    let val = UnsignedSigned::<$i, $u> { unsigned: *self };
                    unsafe {
                        bits.push(&val.signed);
                    }
                }

                fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
                    let mut val: $u = 0;
                    (&mut val).extract_from_packed_bits(bits);
                    let val = UnsignedSigned::<$i, $u> { signed: val };
                    unsafe {
                        *self = val.unsigned;
                    }
                }
            }
        )*
    }
}

impl_for!(
    i16:u16 i32:u32 i64:u64 i128:u128
);