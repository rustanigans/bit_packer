#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate bit_packer;

use bit_packer::BitPacker;

struct NetPacketHeader {
    protocol_id: u32,
    packet_type: u8,
    // Fragment, Data, Control
    sequence_number: u16,
}

struct ControlPacketPayload {
    command: u8,
    // Connect/Disconnect/Reject/Accept etc..
    control_data_type: u8,
    // None, something else if required, like auth token
    control_data: Box<dyn BitPacker>,
}

struct ControlPacket {
    header: NetPacketHeader,
    data: ControlPacketPayload,
}

struct DataPacketPayload
{
    payload_type: u8,
    // Snapshot, Event, others...
    data: Box<dyn BitPacker>,
}

struct SnapshotHeader {
    snapshot_list: Vec<(u8, u8)> // u8 SnapshotType, u8 Count
}

struct SnapshotPayload {
    header: SnapshotHeader,
    data: Vec<Box<dyn BitPacker>>,
}