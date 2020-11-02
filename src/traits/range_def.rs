use crate::PackedBits;

pub trait RangeDef<T>:Send + Sync
{
    fn new(min: T, max: T) -> Self
        where Self: Sized;
    fn pack(&self, val: T, pb: &mut PackedBits);
    fn restore(&self, val: &mut T, pb: &mut PackedBits);
}
