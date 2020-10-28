use super::*;
use std::ops::{Shl, Shr};

pub(super) fn get_u_from_pb<T: Shr<u32, Output = T> + Copy + NumByteConverter<T>>(pb: &mut PackedBits,
                                                                                  zeros: u32)
                                                                                  -> T
{
    let mut val_vec = pb.take_bits(T::BIT_SIZE - zeros as usize);
    if val_vec.len() < T::BYTE_SIZE
    {
        let ext = T::BYTE_SIZE - val_vec.len();
        val_vec.append(&mut vec![0u8; ext])
    }
    (unsafe { T::from_be_byte_vec(val_vec) }) >> zeros
}

pub(super) fn get_vec_from_u<T: Shl<u32, Output = T> + Copy + NumByteConverter<T>>(
    val: T,
    zeros: u32)
    -> (Vec<u8>, usize)
{
    let mut v = (val << zeros).into_be_byte_vec();
    v.truncate(v.len() - (zeros as usize / 8));
    (v, zeros as usize % 8)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn get_vec_from_u_should_shift_u8()
    {
        let val = 255_u8;
        let (v, _z) = get_vec_from_u(val, 4);

        assert_eq!(255 << 4, v[0]);
    }

    #[test]
    fn get_vec_from_u_should_shift_u16()
    {
        let val = 255_u16;
        let (v, _z) = get_vec_from_u(val, 12);

        assert_eq!(255 << 4, v[0]);
    }

    #[test]
    fn get_vec_from_u_should_shift_u16_2()
    {
        let val = u16::MAX;
        let (v, _z) = get_vec_from_u(val, 4);

        assert_eq!(&[255, 255 << 4], &v[..]);
    }

    #[test]
    fn get_vec_from_u_when_u32_and_zeros_24_should_truncate_vector_to_len_1()
    {
        let val = 255_u32;
        let (v, _z) = get_vec_from_u(val, 24);

        assert_eq!(1, v.len());
    }

    #[test]
    fn get_vec_from_u_when_u8_should_not_truncate_vector()
    {
        let val = 255_u8;
        let (v, _z) = get_vec_from_u(val, 4);

        assert_eq!(1, v.len());
    }

    #[test]
    fn get_vec_from_u_should_truncate_vector()
    {
        let val = 255_u16;
        let (v, _z) = get_vec_from_u(val, 9);

        assert_eq!(1, v.len());
    }

    #[test]
    fn get_vec_from_u_should_zeros_mod_eight()
    {
        let val = 255_u16;
        let (_v, z) = get_vec_from_u(val, 9);

        assert_eq!(1, z);
    }

    #[test]
    fn get_u_from_pb_should_get_7_u8()
    {
        let mut pb = PackedBits::default();
        pb.append(&mut vec![7 << 5], 5);

        let result: u8 = get_u_from_pb(&mut pb, 5);
        assert_eq!(7, result);
    }

    #[test]
    fn get_u_from_pb_should_get_256_u16()
    {
        let mut pb = PackedBits::default();
        pb.append(&mut (256_u16 << 7).into_be_byte_vec(), 7);

        let result: u16 = get_u_from_pb(&mut pb, 7);
        assert_eq!(256, result);
    }

    #[test]
    fn get_u_from_pb_should_get_255_u32()
    {
        let mut pb = PackedBits::default();
        pb.append(&mut (255_u32 << 24).into_be_byte_vec(), 24);

        let result: u32 = get_u_from_pb(&mut pb, 24);
        assert_eq!(255, result);
    }
}
