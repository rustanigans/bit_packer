pub struct IntRangeDef<T>
{
    pub(crate) min: T,

    // Only need this for checks
    pub(crate) max: T,

    pub(crate) zeros: u32,
}