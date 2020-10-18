#[cfg(test)]
mod tests
{
    #[test]
    fn it_works()
    {
        assert_eq!(2 + 2, 4);
    }
}

extern crate bit_packer;

use bit_packer::{BitPacker, PackedBits};

struct NetPacketHeader
{
    protocol_id: u32,
    // Fragment, Data, Control = 2bits
    packet_type: u8,
    sequence_number: u16,
    confirmed_number: u16,
}

struct Packet<Coffee>
where Coffee: BitPacker
{
    header: NetPacketHeader,
    data: Coffee,
}

struct ControlPacket
{
    header: NetPacketHeader,
    data: ControlPacketPayload,
}

struct ControlPacketPayload
{
    // Connect/Disconnect/Reject/Accept/Ack etc..
    command: u8,
    // None, something else if required, like auth token
    control_data_type: u8,
    control_data: Box<dyn BitPacker>,
}

struct Ack
{
    sequence_number: u16,    //500 500-1 500-2  500-8 500-32
    previous_sequences: u32, // 0b 0000 0000 0000 0000 0000 0000 1000 0011
}

struct DataPacker
{
    header: NetPacketHeader,
    data: DataPacketPayload,
}

struct DataPacketPayload
{
    payload_type: u8,
    // Snapshot, Event, others...
    data: Box<dyn BitPacker>,
}

struct SnapshotPayload
{
    header: SnapshotHeader,
    data: Vec<Box<dyn BitPacker>>,
}

struct SnapshotHeader
{
    snapshot_list: Vec<(u8, u8)>, // u8 SnapshotType, u8 Count
}
