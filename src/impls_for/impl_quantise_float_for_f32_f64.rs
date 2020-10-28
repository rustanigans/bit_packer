use crate::QuantiseFloat;

macro_rules! impl_for
{
    ($($t:ty:$t2:ty)*) =>
    {
        $(
            impl QuantiseFloat<$t2> for $t
            {
                fn quantise(&self, scale: u32) -> $t2
                {
                    let scaled = self * scale as $t;
                    debug_assert!(!scaled.is_nan(), "Quantised value is NaN");
                    debug_assert!(!scaled.is_infinite(), "Quantised value is not finite");
                    debug_assert!(
                        scaled <= <$t2>::MAX as $t && scaled >= <$t2>::MIN as $t,
                        "Quantised value should fit in integer type ({})",
                        stringify!(i32)
                    );
                    scaled as $t2
                }
            }
        )*
    }
}

impl_for!(f32:i32 f32:u32 f64:i64 f64:u64);

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn quantise_very_small_number_as_expected()
    {
        let f = 0.000000012_f32;
        let q: i32 = f.quantise(1000000000);

        assert_eq!(12, q);
    }

    #[test]
    fn quantise_very_large_number_as_expected()
    {
        let f = u32::MAX as f32;
        let q: u32 = f.quantise(1);

        assert_eq!(u32::MAX, q);
    }

    #[test]
    fn quantise_negative_number_as_expected()
    {
        let f = -0.000000012_f32;
        let q: i32 = f.quantise(1000000000);

        assert_eq!(-12, q);
    }

    #[test]
    #[should_panic]
    fn quantise_when_infinite_should_panic()
    {
        let f = f32::INFINITY;
        let _: u32 = f.quantise(10);
    }

    #[test]
    #[should_panic]
    fn quantise_when_outside_integer_bounds_should_panic()
    {
        let f = u32::MAX as f32 + 1.0;
        let _: u32 = f.quantise(10);
    }

    #[test]
    #[should_panic]
    fn quantise_when_is_nan_should_panic()
    {
        let f = f32::NAN;
        let _: u32 = f.quantise(10);
    }
}
