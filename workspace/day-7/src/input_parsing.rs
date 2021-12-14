use crate::models::Input;
use std::error::Error;
use std::fs;

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let mut crab_positions = content
        .trim()
        .split(',')
        .map(|position| position.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    crab_positions.sort();

    Ok(Input { crab_positions })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::Input;

    #[test]
    fn parse_example_case() {
        let content = "16,1,2,0,4,2,7,1,2,14
"
        .to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(
            input,
            Input {
                crab_positions: vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16]
            }
        );
    }
}
