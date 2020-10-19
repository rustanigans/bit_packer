pub struct UIntRangeDef<T>
{
    pub(super) min: T,

    // Only need this for checks
    pub(super) max: T,

    pub(super) zeros: u32,
}

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