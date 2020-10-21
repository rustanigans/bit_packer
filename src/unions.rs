pub(crate) union UnsignedSigned<U,S>
where U: Copy, S: Copy
{
    pub(crate) unsigned:U,
    pub(crate) signed:S
}

pub(crate) union FloatUInt<F,U>
    where U: Copy, F: Copy
{
    pub(crate) float:F,
    pub(crate) uint:U
}