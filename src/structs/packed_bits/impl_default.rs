use crate::PackedBits;

impl Default for PackedBits
{
    fn default() -> Self
    {
        PackedBits { trailing_zeros: 0,
                     bytes:          vec![] }
    }
}
