#![allow(clippy::suspicious_else_formatting)]
#[macro_use]
mod extract_payload;

extern crate bit_packer;

use crate::PacketPayloadOption::{Incoming, Outgoing};
use bit_packer::*;

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
            protocol_id: 231,
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
        bits.shift(&mut self.header);

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

impl Default for ControlPacketPayload
{
    fn default() -> Self
    {
        ControlPacketPayload {
            control_command: 0,
            control_command_data: PacketPayloadOption::None,
        }
    }
}

impl BitPacker for ControlPacketPayload
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { unimplemented!() }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) { unimplemented!() }
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
        bits.shift(&mut self.payload_type);

        self.data = Incoming(Box::new(bits.swap_empty()));
    }
}

impl Default for DataPacketPayload
{
    fn default() -> Self
    {
        DataPacketPayload {
            payload_type: 0,
            data: PacketPayloadOption::default(),
        }
    }
}

struct SnapshotHeader
{
    snapshot_list: Vec<(u16, u16)>, // u8 SnapshotType, u8 Count
}

struct SnapshotPayload
{
    header: SnapshotHeader,
    data: Vec<Box<dyn BitPacker>>,
}

struct Tree
{
    location: u16,
}

impl BitPacker for Tree
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { bits.push(&self.location) }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) { bits.shift(&mut self.location) }
}

#[test]
fn testtt()
{
    let tree = Tree { location: 0 };

     //server.send_snapshots(t Box::new(tree));
    //send_snapshots()
}


pub type SnapshotListItem = (u16, Box<dyn BitPacker>);

fn send_snapshots(mut list: Vec<SnapshotListItem>)
{
    // Sort by type
    list.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut header_list: Vec<(u16, u16)> = vec![];
    // count the number of each type
    let mut current_count = 0u16;
    let mut current_type = 0u16;
    let mut first = true;
    let mut snapshots: Vec<Box<dyn BitPacker>> = Vec::with_capacity(list.len());

    for entry in list
    {
        if current_type != entry.0
        {
            if !first
            {
                header_list.push((current_type, current_count));
            }
            else
            {
                first = false;
            }
            current_count += 1;
            current_type = entry.0;
        }
        else
        {
            current_count += 1;
        }
        snapshots.push(entry.1);
    }
    header_list.push((current_type, current_count));

    // Add an entry to header for each type

    let payload = SnapshotPayload {
        header: SnapshotHeader {
            snapshot_list: header_list,
        },
        data: snapshots
    };
}

impl BitPacker for SnapshotPayload
{
    fn add_to_packed_bits(&self, bits: &mut PackedBits) { unimplemented!() }

    fn extract_from_packed_bits(&mut self, bits: &mut PackedBits) { unimplemented!() }
}

impl Default for NetPacketHeader
{
    fn default() -> Self
    {
        NetPacketHeader {
            protocol_id: 0,
            sequence_number: 0,
            confirmed_number: 0,
            packet_type: 0,
        }
    }
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

    pub fn from_buff(buff: Vec<u8>) -> NetPacket
    {
        let mut pkt = NetPacket::default();
        pkt.unpack(buff);
        pkt
    }
}

mod ttt;

#[test]
fn incoming_builder_test()
{
    let buff: Vec<u8> = vec![];
    let mut pkt = NetPacketBuilder::from_buff(buff);

    extract_payload!(pkt.payload, control_payload, ControlPacketPayload);

    /*let mut control_payload = ControlPacketPayload::default();

    match pkt.payload
    {
        Incoming(ref mut payload) => payload.shift(&mut control_payload),
        _ => panic!(),
    }*/
    // Can now move into result
    //Ok((pkt, control_payload));
    //return pkt.header;

    //packet_payload.payload_type.
}

#[test]
fn builder_test()
{
    let payload = DataPacketPayload::default();
    let mut pkt = NetPacketBuilder::new()
        .with_header(NetPacketHeader::default())
        .with_payload(Outgoing(payload.into())).pkt;

    for i in 1..100
    {
        pkt.header.sequence_number = i;

        let bytes = pkt.pack();
        //send_data(pb.bytes());
    }
}
