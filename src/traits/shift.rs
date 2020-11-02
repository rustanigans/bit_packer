use crate::BitPacker;

pub trait Shift: Send
{
    #[deprecated(since = "0.1.1",
                 note = "Use PackedBits.take_front instead. If you must use this (i.e. you need to use a 'dyn Shift') let @Sibz know of the use case")]
    fn shift(&mut self, val: &mut dyn BitPacker);
}
