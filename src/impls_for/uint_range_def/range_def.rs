use crate::RangeDef;
use crate::UIntRangeDef;

impl RangeDef<u8> for UIntRangeDef<u8>
{
    fn new(min: u8, max: u8) -> Self{
        UIntRangeDef ::new(min, max)
    }
}