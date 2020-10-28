pub trait QuantiseFloat<T>
{
    fn quantise(&self, scale: u32) -> T;
}
