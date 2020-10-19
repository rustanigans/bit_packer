use crate::PackedBits;

macro_rules!test_append{
    ($($name:ident, $initial:expr, $append:expr, $tz:expr, $expected:expr, $expected_tz:expr)*) =>
    {
        $(
            #[test]
            fn $name() {
                let mut pb = PackedBits::from($initial);
                pb.append($append, $tz);

                assert_eq!($expected, pb.bytes());
                assert_eq!($expected_tz, pb.trailing_zeros());
            }

        )*
    }
}
test_append!(
append_4bit_to_4bit, (vec![0b11110000], 4), &mut vec![0b11110000], 4, &[255], 0
append_8bit_to_4bit, vec![255], &mut vec![0b11110000], 4, &[255, 0b11110000], 4
append_4bit_to_8bit, vec![255], &mut vec![0b11110000], 4, &[255, 0b11110000], 4
append_7bit_to_8bit, vec![255], &mut vec![0b11111110], 1, &[255, 0b11111110], 1
append_15bit_to_16bit, vec![255,255], &mut vec![255, 254], 1, &[255,255,255,254], 1
append_15bit_to_15bit, (vec![255,254], 1), &mut vec![255, 254], 1, &[255,255,255,252], 2
);
