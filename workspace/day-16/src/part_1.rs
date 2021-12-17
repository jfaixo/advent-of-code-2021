use crate::BITSPacket;

pub fn sum_header_versions(packet: &BITSPacket) -> usize {
    match packet {
        BITSPacket::LiteralPacket { header, value } => {
            header.version as usize
        }
        BITSPacket::OperatorPacket { header, sub_packets } => {
            header.version as usize + sub_packets.iter().map(|packet| sum_header_versions(packet)).sum::<usize>()
        }
    }
}
