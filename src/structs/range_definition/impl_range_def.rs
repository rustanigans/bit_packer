use super::{
    range_definition_helpers::{get_u_from_pb, get_vec_from_u},
    *,
};

#[cfg(test)]
mod tests
{
    use super::*;

    macro_rules! test_range {
        ($range:expr,$val:expr => $t:ident($tz:expr, $shifted:expr, $min:expr)) => {
            #[test]
            fn should_produce_pb_with_correct_tz()
            {
                let val: $t = $val;
                let rd = RangeDefinition::new(0, $range);
                let mut pb = PackedBits::default();
                rd.pack(val, &mut pb);

                assert_eq!($tz, pb.trailing_zeros());
            }

            #[test]
            fn should_produce_pb_containing_shifted_values()
            {
                let val: $t = $val;
                let rd = RangeDefinition::new(0, $range);
                let mut pb = PackedBits::default();
                rd.pack(val, &mut pb);

                assert_eq!($shifted, pb.bytes());
            }
            #[test]
            fn should_store_value_less_min()
            {
                let val: $t = $min + $val;
                let rd = RangeDefinition::new($min, $min + $range);
                let mut pb = PackedBits::default();
                rd.pack(val, &mut pb);

                assert_eq!($shifted, pb.bytes());
            }
        };
    }

    macro_rules! test_ranges
    {
        ($range_word:ident-($range:expr, $shifted:expr, $val:expr) => $($t:ident($tz:expr, $min:expr))*) =>
        {
            mod $range_word
            {
                use super::*;

                $(
                    mod $t {
                        use super::*;
                        test_range!($range,$val => $t($tz, $shifted, $min));
                    }
                )*
            }
        }
    }

    test_ranges!(
        seven-(7,&[(7/2)<<5], 7/2) =>
            u8(5, 10) u16(5, 300) u32(5, 70000) u64(5, (u32::MAX as u64 + 1)) u128(5, (u64::MAX as u128 + 1))
            i8(4, 10) i16(4, 300) i32(4, 70000) i64(4, (i32::MAX as i64 + 1))
    );
    test_ranges!(
        three_hundred-(300,&[75,0], 300/2) =>
            u16(7, 300) u32(7, 70000) u64(7, (u32::MAX as u64 + 1)) u128(7, (u64::MAX as u128 + 1))
            i16(6, 300) i32(6, 70000) i64(6, (i32::MAX as i64 + 1))
    );
    test_ranges!(
        seventy_thousand-(70000,&[68,92,0], 70000/2) =>
            u32(7, 70000) u64(7, (u32::MAX as u64 + 1)) u128(7, (u64::MAX as u128 + 1))
            i32(6, 70000) i64(6, (i32::MAX as i64 + 1))
    );

    test_ranges!(
        seven_neg-(7,&[(7/2)<<5], 7/2) =>
            i8(4, -10) i16(4, -300) i32(4, -70000) i64(4, (i32::MIN as i64 - 1))
    );

    test_ranges!(
        seventy_thousand_neg-(70000,&[68,92,0], 70000/2) =>
            i32(6, -70000) i64(6, (i32::MIN as i64 + 1))
    );
}

macro_rules! impl_unsigned
{
($($t:ty)*) => {
    $(
        impl RangeDef<$t> for RangeDefinition<$t>
        {
            fn new(min: $t, max: $t) -> Self
            {
                RangeDefinition {
                    min,
                    #[cfg(debug_assertions)] max,
                    zeros: (max - min).leading_zeros(),
                }
            }

            fn pack(&self, val: $t, pb: &mut PackedBits)
            {
                debug_assert!(val >= self.min && val <= self.max, "Value outside range");
                let (mut v,z) = get_vec_from_u(val - self.min, self.zeros);
                pb.append(&mut v, z);
            }

            fn restore(&self, val: &mut $t, pb: &mut PackedBits)
            {
                *val = get_u_from_pb::<$t>(pb, self.zeros) + self.min;
            }
        }
    )*
}
}

impl_unsigned!(u8 u16 u32 u64 u128);

macro_rules! impl_signed
{
($($t:ty:$t2:ty:$t3:ty)*) =>
{
    $(

        impl RangeDef<$t> for RangeDefinition<$t>
        {
            fn new(min: $t, max: $t) -> Self
            {
                debug_assert!(max>min, "New range: max must be > than min");
                RangeDefinition {
                    min,
                    #[cfg(debug_assertions)] max,
                    zeros: (((max as $t3).wrapping_sub(min as $t3)) as $t2).leading_zeros() - 1,
                }
            }

            fn pack(&self, mut val: $t, pb: &mut PackedBits)
            {
                debug_assert!(val >= self.min && val <= self.max, "Value outside range");
                val -= self.min;
                let val = UnsignedSigned::<$t2, $t> { signed: val };
                let val = unsafe { val.unsigned.rotate_left(1) };
                let (mut v,z) = get_vec_from_u(val, self.zeros);
                pb.append(&mut v, z);
            }

            fn restore(&self, val: &mut $t, pb: &mut PackedBits)
            {
                let us = UnsignedSigned {
                    unsigned: get_u_from_pb::<$t2>(pb, self.zeros).rotate_right(1)
                };
                *val = unsafe { us.signed };
                *val += self.min;
            }
        }

    )*
}
}

impl_signed!(i8:u8:i16 i16:u16:i32 i32:u32:i64 i64:u64:i128);
