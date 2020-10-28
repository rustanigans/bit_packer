use crate::PackedBits;

#[test]
#[should_panic]
fn when_empty_should_panic()
{
    let mut pb = PackedBits::default();

    pb.take_byte();

    // TODO make panic message clearer
}

#[test]
fn when_one_byte_in_pb_should_get_that_byte()
{
    let expected = 0b10000001_u8;
    let mut pb = PackedBits::from_bytes(vec![expected]);

    let byte = pb.take_byte();

    assert_eq!(expected, byte);
}

#[test]
fn when_multiply_bytes_in_pb_should_get_first_byte()
{
    let expected = 0b10000001_u8;
    let mut pb = PackedBits::from_bytes(vec![expected, 255, 255]);

    let byte = pb.take_byte();

    assert_eq!(expected, byte);
}
