use crate::BitPacker;

pub trait BoxBitPacker: BitPacker
{
    fn into_box(self) -> Box<dyn BitPacker>;
}