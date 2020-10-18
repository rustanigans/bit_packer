use crate::Push;
use crate::PackedBits;

impl Push<u8> for PackedBits
{
    fn push(&mut self, val: u8) {
        self.add_byte(val)
    }
}

macro_rules! impl_for{
($($i:ident:$n:expr)*) => {
        $(
            impl Push<$i> for PackedBits
            {
                fn push(&mut self, val: $i) {
                    self.push(
                        &mut unsafe {
                            Vec::from_raw_parts(
                                val.to_be_bytes().as_mut_ptr(), $n, $n)
                                });
                }
            }
        )*
    }
}

impl_for!(u16:2 u32:4 u64:8 u128:16);