use crate::models::Input;
use std::error::Error;
use std::fs;

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let fish_timers = content
        .trim()
        .split(',')
        .map(|timer| timer.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut fish_per_timer = [0; 9];
    for fish in fish_timers {
        fish_per_timer[fish] += 1;
    }

    Ok(Input { fish_per_timer })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::Input;

    #[test]
    fn parse_example_case() {
        let content = "3,4,3,1,2
"
        .to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(
            input,
            Input {
                fish_per_timer: [0, 1, 1, 2, 1, 0, 0, 0, 0]
            }
        );
    }
}
