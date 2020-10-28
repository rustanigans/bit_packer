use crate::BitPacker;

pub trait Shift: Send
{
    fn shift(&mut self, val: &mut dyn BitPacker);
}
