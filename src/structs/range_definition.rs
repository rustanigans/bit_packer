use crate::{BitPacker, PackedBits, RangeDef};
use std::ops::Deref;

pub struct RangeDefinition<T>
{
    pub(crate) min: T,

    // Only need this for checks
    pub(crate) max: T,

    pub(crate) zeros: u32,
}
/*type RangedNum<T,U> = (T,RangeDef<U>);
impl BitPacker for RangedNum<u8,u8>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) {
        let x = self.1.;

    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) { unimplemented!() }
}
*/