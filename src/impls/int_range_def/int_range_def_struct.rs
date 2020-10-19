pub struct IntRangeDef<T>
{
    pub(super) min: T,

    // Only need this for checks
    pub(super) max: T,

    pub(super) zeros: u32,
}