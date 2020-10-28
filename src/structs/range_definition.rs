use crate::*;
mod impl_range_def;
mod range_definition_helpers;

pub struct RangeDefinition<T>
{
    min:   T,
    // Only need this for checks
    #[cfg(debug_assertions)]
    max:   T,
    zeros: u32
}
