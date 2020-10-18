pub trait RangeDef<T>{
    fn new(min:T, max:T) -> Self;
}