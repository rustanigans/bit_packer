pub mod uint_range_def_struct;

pub use uint_range_def_struct::UIntRangeDef;

impl<T> UIntRangeDef<T> {
    pub fn new(min:T, max:T) -> Self
    {
        UIntRangeDef {
            min,
            max,
            zeros: 0
        }
    }
}