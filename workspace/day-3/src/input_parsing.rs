use std::fs;
use crate::models::{Input};

pub fn parse_file(file_path: String) -> Input {
    let file_content = fs::read_to_string(file_path)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Input {
    let mut bit_count = 0;
    let data = content.lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            }
            else {
                bit_count = line.len();
                let mut value = 0;
                for (i, c) in line.chars().enumerate() {
                    if c == '1' {
                        value |= 0b1 << (bit_count - 1 - i);
                    }
                }
                Some(value)
            }
        }).collect();

    Input {
        bit_count,
        data
    }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{Input};

    #[test]
    fn parse_example_case() {
        let content = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
".to_string();

        let input = parse_string(content);

        assert_eq!(input, Input {
            bit_count: 5,
            data: vec![
                0b00100,
                0b11110,
                0b10110,
                0b10111,
                0b10101,
                0b01111,
                0b00111,
                0b11100,
                0b10000,
                0b11001,
                0b00010,
                0b01010,
            ]
        });
    }
}
