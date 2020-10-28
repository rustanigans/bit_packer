use crate::{BitPacker, BoxBitPacker};

impl<T: 'static + BitPacker> From<T> for Box<dyn BitPacker>
{
    fn from(item: T) -> Self
    {
        item.into_box()
    }
}
