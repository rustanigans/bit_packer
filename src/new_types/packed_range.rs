use crate::RangeDef;

pub type PackedRange<'a, T> = (T, &'a dyn RangeDef<T>);
