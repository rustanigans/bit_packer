use crate::{BitPacker, BoxBitPacker};

impl<T: 'static + BitPacker> BoxBitPacker for T
{
    fn into_box(self) -> Box<dyn BitPacker>
    {
        Box::new(self)
    }
}
