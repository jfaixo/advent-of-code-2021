use crate::BITSPacket;

pub fn compute_packet_value(packet: &BITSPacket) -> u64 {
    match packet {
        BITSPacket::LiteralPacket { header, value } => {
            *value
        }
        BITSPacket::OperatorPacket { header, sub_packets } => {
            match header.packet_type {
                0 => {
                    sub_packets.iter().map(|packet| compute_packet_value(packet)).sum()
                }
                1 => {
                    sub_packets.iter().map(|packet| compute_packet_value(packet)).fold(1, |a, b| a * b)
                }
                2 => {
                    sub_packets.iter().map(|packet| compute_packet_value(packet)).min().unwrap()
                }
                3 => {
                    sub_packets.iter().map(|packet| compute_packet_value(packet)).max().unwrap()
                }
                5 => {
                    let first_packet = compute_packet_value(&sub_packets[0]);
                    let second_packet = compute_packet_value(&sub_packets[1]);
                    if first_packet > second_packet { 1 } else { 0 }
                }
                6 => {
                    let first_packet = compute_packet_value(&sub_packets[0]);
                    let second_packet = compute_packet_value(&sub_packets[1]);
                    if first_packet < second_packet { 1 } else { 0 }
                }
                7 => {
                    let first_packet = compute_packet_value(&sub_packets[0]);
                    let second_packet = compute_packet_value(&sub_packets[1]);
                    if first_packet == second_packet { 1 } else { 0 }
                }
                _ => {
                    panic!()
                }
            }
        }
    }
}
