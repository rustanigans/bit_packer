use crate::{NetPacketBuilder, NetPacketHeader, DataPacketPayload};
use crate::PacketPayloadOption::{Incoming, Outgoing};
use bit_packer::{PackedBits, BitPacker};

#[test]
fn tttt() {
    let buff: Vec<u8> = vec![];
    let mut pkt = NetPacketBuilder::new()
        .with_header(NetPacketHeader::default())
        .with_payload( Outgoing(Box::new(DataPacketPayload::default())));

    let mut pb = PackedBits::new();
    pkt.pkt.add_to_packed_bits(&mut pb);

    //extract_payload!(pkt.payload, control_payload, u8);
}
