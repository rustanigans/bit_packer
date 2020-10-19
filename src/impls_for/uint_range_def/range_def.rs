use crate::RangeDef;
use crate::UIntRangeDef;

impl RangeDef<u8> for UIntRangeDef<u8>
{
    fn new(min: u8, max: u8) -> Self{
        UIntRangeDef ::new(min, max)
    }

    fn pack(val: u8) -> (Vec<u8>, usize) {
        unimplemented!()
    }

    fn restore(val: Vec<u8>) -> u8 {
        unimplemented!()
    }
}