use crate::BitPacker;

pub trait Shift {

    fn shift(&mut self, val: &mut dyn BitPacker);
}