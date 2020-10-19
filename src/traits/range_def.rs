pub trait RangeDef<T>
{
    fn new(min: T, max: T) -> Self;
    fn pack(val: T) -> (Vec<u8>, usize);
    fn restore(val: Vec<u8>) -> T;
}
