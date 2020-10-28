use crate::utils::shift_right::shift_right;

macro_rules! test_shift_right {
($($name:ident, $shift: expr, $tz: expr, $value: expr, $expected: expr, $expected_tz:expr)*) =>
    {
        $(
            #[test]
            fn $name() {
                let mut bytes  = $value;
                let result = shift_right(&mut bytes, $shift, $tz);
                assert_eq!($expected, &bytes[..]);
                assert_eq!($expected_tz, result);
            }
        )*
    }
}

test_shift_right!(
    shift_8bits_by_8bits, 8, 0, vec![255], &[0,255], 0
    shift_4bits_by_8bits, 8, 4, vec![0b11110000], &[0,0b11110000], 4
    shift_4bits_by_12bits, 12, 4, vec![0b11110000], &[0,0b00001111], 0
    shift_17bits_by_15bits,  15, 7,
        vec![0b11010101, 0b01010101, 0b10000000],
        &[0,0b00000001,0b10101010, 0b10101011], 0
    shift_16bits_by_16bits, 16, 0, vec![255;2], &[0,0,255,255], 0
    shift_12bits_by_4bits, 4, 4, vec![255,0b11110000], &[0b00001111,255], 0
    shift_8bits_by_1bits, 1, 0, vec![255], &[255>>1,128], 7
    shift_2bits_by_3bits, 3, 6, vec![0b11000000], &[0b00011000], 3
    shift_15bits_by_15bits, 15, 1, vec![255,254], &[0,1,255,252], 2
);
