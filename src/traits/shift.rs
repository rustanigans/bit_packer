use crate::BitPacker;

pub trait Shift {

    fn shift<T: BitPacker>(&mut self, val: &mut T);
}