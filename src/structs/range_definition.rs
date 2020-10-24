use crate::*;
mod range_definition_helpers;
mod impl_range_def;

pub struct RangeDefinition<T>
{
    min: T,

    // Only need this for checks
    #[cfg(debug_assertions)]
    max: T,

    zeros: u32,
}


