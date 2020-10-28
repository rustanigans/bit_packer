use crate::PackedBits;

macro_rules! take_bits
{
    ($($name:ident, $in:expr, $count:expr, $expected_tz:expr, $expected:expr, $expected_remaining:expr)*) =>
    {
        $(
            #[test]
            fn $name() {
                let mut pb = PackedBits::from($in);
                let result = pb.take_bits($count);
                let remaining_bytes = pb.bytes();
                assert_eq!($expected_tz, pb.trailing_zeros());
                assert_eq!($expected, &result[..]);
                assert_eq!($expected_remaining, remaining_bytes);
            }
        )*
    }
}

take_bits!(
    take_4_from_8, vec![255], 4, 4, &[255<<4], &[255<<4]
    take_8_from_8, vec![255], 8, 0, &[255], <Vec<u8>>::new()
    take_1_from_16, vec![255,255], 1, 1, &[128],  &[255, 255<<1]
    take_4_from_16, vec![255,255], 4, 4, &[255<<4],  &[255, 255<<4]
    take_7_from_16, vec![255,255], 7, 7, &[255<<1],  &[255, 255<<7]

    take_8_from_16, vec![255,255], 8, 0, &[255],  &[255]
    take_9_from_16, vec![255,255], 9, 1, &[255, 255<<7],  &[255<<1]
    take_15_from_16, vec![255,255], 15, 7, &[255, 255<<1],  &[255<<7]

    take_16_from_32, vec![255,255,255,255], 16, 0, &[255, 255],  &[255,255]
    take_8_from_32, vec![255,255,255,255], 8, 0, &[255],  &[255,255,255]
    take_24_from_32, vec![255,255,255,255], 24, 0, &[255, 255, 255],  &[255]

    take_9_from_32, vec![255,255,255,255], 9, 1, &[255, 255<<7],  &[255,255,255<<1]
    take_30_from_32, vec![255,255,255,255], 30, 6, &[255, 255, 255, 255<<2],  &[255<<6]
);
