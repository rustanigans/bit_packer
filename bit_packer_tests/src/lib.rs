extern crate bit_packer;

use bit_packer::{BitPacker, PackedBits, Push};
use std::{borrow::BorrowMut, rc::Rc};

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

trait Newable
{
    fn new() -> Self;
}

struct Packet<Coffee>
{
    header: NetPacketHeader,
    payload: Coffee,
}

impl<T> Packet<T>
where T: BitPacker + Default
{
    pub fn new() -> Packet<T>
    {
        Packet {
            header: NetPacketHeader::default(),
            payload: T::default(),
        }
    }
}

impl<T: BitPacker> BitPacker for Packet<T>
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        bits.push(&self.header.protocol_id);
        bits.push(&self.header.sequence_number);
        bits.push(&self.payload);
        //bits.push( );
    }
}

struct ControlPacketPayload<T>
{
    // Connect/Disconnect/Reject/Accept/Ack etc..
    command: u8,
    // None, something else if required, like auth token
    control_data_type: u8,
    control_data: T,
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
    data: Option<Box<dyn BitPacker>>,
}

impl BitPacker for DataPacketPayload
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits)
    {
        bits.push(&self.payload_type);

        bits.push(self.data.as_ref().unwrap());
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
}

struct SnapshotHeader
{
    snapshot_list: Vec<(u8, u8)>, // u8 SnapshotType, u8 Count
}

#[test]
fn new_packet() { let pkt = Packet::<DataPacketPayload>::new(); }

impl Default for NetPacketHeader
{
    fn default() -> Self { unimplemented!() }
}

impl<T: Default> Default for Packet<T>
{
    fn default() -> Self
    {
        Packet {
            header: NetPacketHeader::default(),
            payload: T::default(),
        }
    }
}

struct NetPacketBuilder<T>
{
    pkt: Packet<T>,
}

impl<T: BitPacker + Default> NetPacketBuilder<T>
{
    pub fn new() -> NetPacketBuilder<T>
    {
        NetPacketBuilder {
            pkt: Packet::default(),
        }
    }

    pub fn with_header(mut self, header: NetPacketHeader) -> Self
    {
        self.pkt.header = header;
        self
    }

    pub fn with_payload(mut self, val: T) -> Self
    {
        self.pkt.payload = val;
        self
    }
}

#[test]
fn builder_test()
{
    let payload = DataPacketPayload::default();
    let mut pkt = NetPacketBuilder::new()
        .with_header(NetPacketHeader::default())
        .with_payload(payload)
        .pkt;

    for i in 1..100
    {
        pkt.header.sequence_number = i;

        let mut pb = PackedBits::new();
        pkt.add_to_packed_bits(&mut pb);
        //send_data(pb.bytes());
    }
}
