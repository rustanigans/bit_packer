use crate::PackedBits;

pub trait RangeDef<T>
{
    fn new(min: T, max: T) -> Self;
    fn pack(&self, val: T, pb: &mut PackedBits);
    fn restore(&self, val: &mut T, pb: &mut PackedBits);
}
