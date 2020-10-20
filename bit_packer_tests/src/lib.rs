extern crate bit_packer;

use bit_packer::*;
use crate::PacketPayloadOption::{Incoming, Outgoing};

#[derive(Copy, Clone)]
struct NetPacketHeader
{
    protocol_id: u32,
    // Fragment, Data, Control = 2bits
    packet_type: u8,
    sequence_number: u16,
    confirmed_number: u16,
}

impl NetPacketHeader
{
    fn new() -> Self
    {
        NetPacketHeader {
            protocol_id: 0,
            packet_type: 0,
            sequence_number: 0,
            confirmed_number: 0,
        }
    }
}

impl BitPacker for NetPacketHeader
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        bits.push(&self.protocol_id);
        bits.push(&self.packet_type);
        bits.push(&self.sequence_number);
        bits.push(&self.confirmed_number);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        bits.shift(&mut self.protocol_id);
        bits.shift(&mut self.packet_type);
        bits.shift(&mut self.sequence_number);
        bits.shift(&mut self.confirmed_number)
    }
}

trait Newable
{
    fn new() -> Self;
}

pub enum PacketPayloadOption
{
    None,
    Outgoing(Box<dyn BitPacker>),
    Incoming(Box<dyn Shift>),
}

impl Default for PacketPayloadOption
{
    fn default() -> Self { PacketPayloadOption::None }
}

struct NetPacket
{
    header: NetPacketHeader,
    payload: PacketPayloadOption,
}

impl NetPacket {}

impl BitPacker for NetPacket
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        bits.push(&self.header);
        match &self.payload
        {
            PacketPayloadOption::Outgoing(payload) => bits.push(payload),
            _ => panic!(),
        }
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        bits.shift(  &mut self.header);

        self.payload = Incoming(Box::new(bits.swap_empty()));
    }
}

struct ControlPacketPayload
{
    // Connect/Disconnect/Reject/Accept/Ack etc..
    control_command: u8,
    // None, something else if required, like auth token
    //  control_data_type: u8,
    control_command_data: PacketPayloadOption,
}

struct Ack
{
    sequence_number: u16,    //500 500-1 500-2  500-8 500-32
    previous_sequences: u32, // 0b 0000 0000 0000 0000 0000 0000 1000 0011
}

struct DataPacket
{
    header: NetPacketHeader,
    data: DataPacketPayload,
}

struct DataPacketPayload
{
    payload_type: u8,
    // Snapshot, Event, others...
    data: PacketPayloadOption,
}

impl BitPacker for DataPacketPayload
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        bits.push(&self.payload_type);

        //bits.push(&self.data);
    }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)
    {
        bits.shift( &mut self.payload_type );

        self.data = Incoming(Box::new(bits.swap_empty()));
    }
}

impl Default for DataPacketPayload
{
    fn default() -> Self { unimplemented!() }
}
struct SnapshotPayload
{
    header: SnapshotHeader,
    data: Vec<Box<dyn BitPacker>>,
}

impl BitPacker for SnapshotPayload
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { unimplemented!() }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits)  { unimplemented!() }
}

struct SnapshotHeader
{
    snapshot_list: Vec<(u8, u8)>, // u8 SnapshotType, u8 Count
}

impl Default for NetPacketHeader
{
    fn default() -> Self { unimplemented!() }
}

impl Default for NetPacket
{
    fn default() -> Self
    {
        NetPacket {
            header: NetPacketHeader::default(),
            payload: PacketPayloadOption::default(),
        }
    }
}

struct NetPacketBuilder
{
    pkt: NetPacket,
}

impl NetPacketBuilder
{
    pub fn new() -> NetPacketBuilder
    {
        NetPacketBuilder {
            pkt: NetPacket::default(),
        }
    }

    pub fn with_header(mut self, header: NetPacketHeader) -> Self
    {
        self.pkt.header = header;
        self
    }

    pub fn with_payload(mut self, val: PacketPayloadOption) -> Self
    {
        self.pkt.payload = val;
        self
    }

    pub fn from_buff(buff: Vec<u8>) -> NetPacket {
        let mut pkt = NetPacket::default();
        pkt.unpack(buff);
        pkt
    }
}


#[test]
fn incoming_builder_test() {
    let buff:Vec<u8> = vec![];
    let pkt = NetPacketBuilder::from_buff(buff);


    let mut packet_payload =  DataPacketPayload::default();

    match pkt.payload {
        Incoming(mut payload) => {
            payload.shift(&mut packet_payload)
        }
        _ => {panic!()}
    }
    //packet_payload.payload_type.


}

#[test]
fn builder_test()
{
    let payload = DataPacketPayload::default();
    let mut pkt = NetPacketBuilder::new()
        .with_header(NetPacketHeader::default())
        .with_payload(Outgoing(Box::new(payload)))
        .pkt;

    for i in 1..100
    {
        pkt.header.sequence_number = i;

        let bytes = pkt.pack();
        //send_data(pb.bytes());
    }
}

