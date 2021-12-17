use std::error::Error;
use crate::BITSPacket::LiteralPacket;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct BITSPacketHeader {
    pub version: u8,
    pub packet_type: u8,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BITSPacket {
    LiteralPacket {
        header: BITSPacketHeader,
        value: u64,
    },
    OperatorPacket {
        header: BITSPacketHeader,
        sub_packets: Vec<BITSPacket>
    }
}

struct RawData {
    data: Vec<u64>,
    bit_count: usize,
}

impl BITSPacket {
    pub fn parse_string(content: String) -> Result<BITSPacket, Box<dyn Error>> {
        // Transform the message in raw u64 values
        let lines = content.split_ascii_whitespace().collect::<Vec<_>>();
        let u64_count = lines[0].len() / 16 + 1;
        let raw_data = (0..u64_count).map(|i| {
            let slice_start_index = i * 16;
            if slice_start_index + 16 > lines[0].len() {
                let mut slice = lines[0][slice_start_index..].to_string();

                for _i in 0..slice_start_index + 16 - lines[0].len() {
                    slice.push('0');
                }
                u64::from_str_radix(&slice, 16)
            }
            else {
                u64::from_str_radix(&lines[0][slice_start_index..slice_start_index + 16], 16)
            }
        }).collect::<Result<Vec<_>, _>>()?;

        let (packet, _) = parse_packet(&RawData { data: raw_data, bit_count: lines[0].len() * 4 }, 0);
        Ok(packet)
    }
}

fn parse_packet(raw_data: &RawData, start_bit_index: usize) -> (BITSPacket, usize) {
    // Extract the version
    let ref mut current_bit_index = start_bit_index.clone();
    let version = extract_bits(raw_data, current_bit_index, 3) as u8;
    let packet_type = extract_bits(raw_data, current_bit_index, 3) as u8;
    let header = BITSPacketHeader { version, packet_type };

    let packet = match header.packet_type {
        4 => {
            // Parse a litteral packet
            let mut value = 0;
            loop {
                let continue_flag = extract_bits(raw_data, current_bit_index, 1);
                let read_value = extract_bits(raw_data, current_bit_index, 4);
                value = value * 16 + read_value;

                if continue_flag == 0 {
                    break;
                }
            }
            LiteralPacket {
                header,
                value
            }
        }
        _ => {
            // Parse an operator packet
            let mut sub_packets = Vec::new();
            let length_type_id = extract_bits(raw_data, current_bit_index, 1);
            if length_type_id == 0 {
                let sub_packets_total_length = extract_bits(raw_data, current_bit_index, 15) as usize;
                let packets_end_bit = *current_bit_index + sub_packets_total_length;

                while *current_bit_index != packets_end_bit {
                    let (packet, end_bit) = parse_packet(raw_data, *current_bit_index);
                    sub_packets.push(packet);
                    *current_bit_index = end_bit;
                }

            } else {
                let sub_packets_count = extract_bits(raw_data, current_bit_index, 11) as usize;
                for _i in 0..sub_packets_count {
                    let (packet, end_bit) = parse_packet(raw_data, *current_bit_index);
                    sub_packets.push(packet);
                    *current_bit_index = end_bit;
                }
            }
            BITSPacket::OperatorPacket { header, sub_packets }
        }
    };

    (packet, *current_bit_index)
}

fn extract_bits(raw_data: &RawData, start:  &mut usize, n: usize) -> u64 {

    let (index, padding) = (*start / 64, *start % 64);
    let value = if padding + n <= 64 {
        // All bits are in the same u64, just extract
        let extracted_value = raw_data.data[index] >> (64 - padding - n);
        let mask = (1 << n) - 1;
        extracted_value & mask
    }
    else {
        // The value is spread over two u64, a bit more complex to extract...
        // First, let's take the remaining bits form the current index
        let mask = (1 << (64 - padding)) - 1;
        let extracted_value_1 = raw_data.data[index] & mask;
        // Then, extract the remaining bits from the next index
        let remaining_bits = n - (64 - padding);
        let extracted_value_2 = raw_data.data[index + 1] >> (64 - remaining_bits);
        let mask = (1 << remaining_bits) - 1;
        let extracted_value_2 = extracted_value_2 & mask;

        // Finally, concat bits
        (extracted_value_1 << remaining_bits) | extracted_value_2
    };
    *start += n;

    value
}

#[cfg(test)]
mod tests {
    use crate::BITSPacket;
    use crate::models::BITSPacketHeader;

    #[test]
    fn parse_example_case_1() {
        let content = "D2FE28
        "
            .to_string();

        let input = BITSPacket::parse_string(content).unwrap();

        assert_eq!(input, BITSPacket::LiteralPacket {
            header: BITSPacketHeader {
                version: 6,
                packet_type: 4
            },
            value: 2021
        });
    }

    #[test]
    fn parse_example_case_2() {
        let content = "38006F45291200
        "
            .to_string();

        let input = BITSPacket::parse_string(content).unwrap();

        assert_eq!(input, BITSPacket::OperatorPacket {
            header: BITSPacketHeader {
                version: 1,
                packet_type: 6
            },
            sub_packets: vec![
                BITSPacket::LiteralPacket {
                    header: BITSPacketHeader {
                        version: 6,
                        packet_type: 4
                    },
                    value: 10
                },
                BITSPacket::LiteralPacket {
                    header: BITSPacketHeader {
                        version: 2,
                        packet_type: 4
                    },
                    value: 20
                },
            ]
        });
    }

    #[test]
    fn parse_example_case_3() {
        let content = "EE00D40C823060
        "
            .to_string();

        let input = BITSPacket::parse_string(content).unwrap();

        assert_eq!(input, BITSPacket::OperatorPacket {
            header: BITSPacketHeader {
                version: 7,
                packet_type: 3
            },
            sub_packets: vec![
                BITSPacket::LiteralPacket {
                    header: BITSPacketHeader {
                        version: 2,
                        packet_type: 4
                    },
                    value: 1
                },
                BITSPacket::LiteralPacket {
                    header: BITSPacketHeader {
                        version: 4,
                        packet_type: 4
                    },
                    value: 2
                },
                BITSPacket::LiteralPacket {
                    header: BITSPacketHeader {
                        version: 1,
                        packet_type: 4
                    },
                    value: 3
                },
            ]
        });
    }

    #[test]
    fn parse_example_case_4() {
        let content = "8A004A801A8002F478
        "
            .to_string();

        let input = BITSPacket::parse_string(content).unwrap();

        assert_eq!(input, BITSPacket::OperatorPacket {
            header: BITSPacketHeader {
                version: 4,
                packet_type: 2
            },
            sub_packets: vec![
                BITSPacket::OperatorPacket {
                    header: BITSPacketHeader {
                        version: 1,
                        packet_type: 2
                    },
                    sub_packets: vec![
                        BITSPacket::OperatorPacket {
                            header: BITSPacketHeader {
                                version: 5,
                                packet_type: 2
                            },
                            sub_packets: vec![
                                BITSPacket::LiteralPacket {
                                    header: BITSPacketHeader {
                                        version: 6,
                                        packet_type: 4
                                    },
                                    value: 15
                                },
                            ]
                        }
                    ]
                }
            ]
        });
    }
}
