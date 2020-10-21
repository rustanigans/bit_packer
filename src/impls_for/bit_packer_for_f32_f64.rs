use crate::{BitPacker, PackedBits, Push, Shift};
use crate::unions::FloatUInt;

macro_rules! impl_for
{
    ($($f:ty:$u:ty)*)=>
    {
        $(
            impl BitPacker for $f {
                fn add_to_packed_bits(&self, bits: &mut PackedBits) {
                    let x: FloatUInt<$f,$u> = FloatUInt { float: *self };
                    unsafe {
                        bits.push(&x.uint);
                    }
                }

                fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
                    let mut x: $u  = 0;
                    bits.shift(&mut x);
                    let x: FloatUInt<$f,$u> = FloatUInt { uint: x};
                    unsafe {
                        *self = x.float;
                    }
                }
            }
        )*
    }
}

impl_for!(f32:u32 f64:u64);