use crate::*;
mod range_definition_helpers;
mod impl_range_def;

pub struct RangeDefinition<T>
{
    pub(crate) min: T,

    // Only need this for checks
    #[cfg(debug_assertions)]
    pub(crate) max: T,

    pub(crate) zeros: u32,
}


