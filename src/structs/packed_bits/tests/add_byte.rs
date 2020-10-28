use crate::PackedBits;

#[test]
fn when_adding_byte_should_add_byte_to_empty_packed_bits()
{
    let mut pb = PackedBits::default();

    pb.add_byte(255);

    assert_eq!(255, pb.bytes[0]);
}

#[test]
fn when_adding_byte_should_add_byte_to_non_empty_packed_bits()
{
    let mut pb = PackedBits::from_bytes(vec![127, 127, 127]);

    pb.add_byte(255);

    assert_eq!(255, pb.bytes[3]);
}

#[test]
fn when_pb_is_not_byte_aligned_should_add_byte_correctly()
{
    let mut pb = PackedBits::from_bytes(vec![144]);
    pb.trailing_zeros = 4;

    pb.add_byte(255);

    assert_eq!(&[159, 240], pb.bytes.as_slice());
}
