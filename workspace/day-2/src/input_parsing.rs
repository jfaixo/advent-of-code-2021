use crate::models::{Input, SubmarineCommand};
use std::fs;

pub fn parse_file(file_path: String) -> Input {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Input {
    let commands = content
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
                let value = parts[1].parse::<i32>().unwrap();
                match parts[0] {
                    "forward" => Some(SubmarineCommand::Forward(value)),
                    "down" => Some(SubmarineCommand::Down(value)),
                    "up" => Some(SubmarineCommand::Up(value)),
                    _ => panic!("Invalid command: {}", line),
                }
            }
        })
        .collect();

    Input { commands }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::Input;
    use crate::models::SubmarineCommand::{Down, Forward, Up};

    #[test]
    fn parse_example_case() {
        let content = "forward 5
down 5
forward 8
up 3
down 8
forward 2
"
        .to_string();

        let input = parse_string(content);

        assert_eq!(
            input,
            Input {
                commands: vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2),]
            }
        );
    }
}
