use crate::{PackedBits, BitPacker};

macro_rules! test_for
{
    ($($n:ident, $initial:expr)*) =>
    {
        $(
            #[test]
            #[allow(clippy::float_cmp)]
            fn $n()
            {
                let mut pb = PackedBits::default();
                let test_val: $n = $initial;

                test_val.add_to_packed_bits(&mut pb);
                assert_eq!(std::mem::size_of::<$n>(),pb.bytes().len(), "pack bits len did not match input type len");

                let mut test_val2 = $n::default();
                test_val2.extract_from_packed_bits(&mut pb);

                assert_eq!(test_val, test_val2);
            }
        )*
    }
}

test_for!(
u8, 42
u16, 420
u32, 420000
u64, u64::MAX
u128, u128::MAX
i8, -42
i16, -420
i32, -420000
i64, i64::MIN
i128, i128::MIN
f32, 4242.42
f64, 424242.42424242
);