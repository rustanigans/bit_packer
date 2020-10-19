use crate::utils::shift_left::shift_left;

macro_rules! shift_test
{
    ($($n:ident, $in:expr, $count:expr, $tz:expr, $expected_trim:expr, $expected_tz:expr, $expected_remainder:expr)*) =>
    {
        $(
            #[test]
            fn $n()
            {
                let mut val: Vec<u8> = $in;
                let result = shift_left(&mut val, $count, $tz);
                assert_eq!($expected_trim,result.0);
                assert_eq!($expected_tz, result.1);
                assert_eq!($expected_remainder, &val[..]);
            }
        )*
    }
}

shift_test!(
shift_8bit_no_tz, vec![255u8], 8, 0, 255, 0, Vec::<u8>::new()
shift_4bit_4_tz, vec![0b11110000], 4, 4, 0b00001111, 0, Vec::<u8>::new()
shift_4bit_2_tz, vec![0b11110100], 4, 2, 0b00001111, 6, vec![0b01000000]
shift_4bit_6_tz, vec![0b11110100, 0b01000000], 4, 6, 0b00001111, 2, vec![0b01000100]
shift_2bit_6_tz, vec![0b01000000], 2, 6, 0b00000001, 0, Vec::<u8>::new()
shift_6bit_from_32bit_2_tz, vec![0b00000100,0,0,4], 6, 2, 0b00000001, 0, vec![0,0,1]
shift_8bit_from_32bit_no_tz, vec![255,1,1,1], 8, 0, 255, 0, vec![1,1,1]
shift_16bit_from_32bit_no_tz, vec![255,128,1,1], 16, 0, 128, 0, vec![1,1]
shift_17bit_from_32bit_no_tz, vec![255,255,128,1], 17, 0, 1, 1, vec![0,2]
);
