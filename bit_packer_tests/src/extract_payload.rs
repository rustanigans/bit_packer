use crate::PacketPayloadOption;
use bit_packer::{BitPacker, NoBits, PackedBits, Push};
#[macro_export]
macro_rules! extract_payload
{
    ($payload_in:expr, $payload_out:ident, $t:ty) => {
        let mut $payload_out = <$t>::default();
        match $payload_in
        {
            Incoming(ref mut payload) => payload.shift(&mut $payload_out),
            _ => panic!(),
        }
    }
}

fn extract_pl<T: Default + BitPacker>(payload_field: &mut PacketPayloadOption) -> T
{
    let mut payload_out = T::default();
    match payload_field {
        PacketPayloadOption::Incoming(ref mut p) =>
            {
                p.shift(&mut payload_out);
            }
        _ => { panic!("tried to extract a payload from a non-incoming option") }
    }
    payload_out
}

#[cfg(test)]
mod tests {
    use super::*;
    use bit_packer::Shift;

    struct SpecialPayload {
        field:u32
    }
    impl BitPacker for SpecialPayload {
        fn add_to_packed_bits(&self, bits: &mut PackedBits) {
            bits.push(&self.field);
        }

        fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) {
            bits.shift(&mut self.field);
        }
    }
    impl Default for SpecialPayload {
        fn default() -> Self {
            SpecialPayload { field: 0}
        }
    }

    #[test]
    fn extract_payload_should_get_payload_out_of_option()
    {
        let mut ppo = PacketPayloadOption::Incoming(Box::new(PackedBits::new()));

        let output: NoBits = extract_pl(&mut ppo);

        // Should not panic
    }

    #[test]
    fn extract_payload_when_has_value_should_get_value_out()
    {
        let mut pb = PackedBits::new();
        pb.push(&42_u8);
        let mut ppo = PacketPayloadOption::Incoming(Box::new(pb));

        let output: u8 = extract_pl(&mut ppo);

        assert_eq!(42, output);
    }

    #[test]
    fn extract_payload_when_has_special_payload_should_get_value_out()
    {
        let mut pb = PackedBits::new();
        pb.push(&SpecialPayload { field: 42 });
        let mut ppo = PacketPayloadOption::Incoming(Box::new(pb));

        let output: SpecialPayload = extract_pl(&mut ppo);

        assert_eq!(42, output.field);
    }

    #[test]
    #[should_panic]
    fn extract_payload_when_is_none_should_panic()
    {
        let mut ppo = PacketPayloadOption::None;
        let output: u8 = extract_pl(&mut ppo);
    }

    #[test]
    #[should_panic]
    fn extract_payload_when_is_outgoing_should_panic()
    {
        let mut ppo = PacketPayloadOption::Outgoing(NoBits::default().into());
        let output: u8 = extract_pl(&mut ppo);
    }
}