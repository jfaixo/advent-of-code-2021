use std::error::Error;
use std::fs;
use crate::models::{Input};

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let octopuses = content.lines()
        .map(|line|{
            line.chars().map(|c| { (c as u8 - '0' as u8) as i8 }).collect::<Vec<i8>>()
        })
        .flatten()
        .collect::<Vec<i8>>();

    let mut input = Input {
        octopuses: [0; 100]
    };
    input.octopuses.copy_from_slice(&*octopuses);

    Ok(input)
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{Input};

    #[test]
    fn parse_example_case() {
        let content = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
".to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(input, Input {
            octopuses: [5,4,8,3,1,4,3,2,2,3,
                2,7,4,5,8,5,4,7,1,1,
                5,2,6,4,5,5,6,1,7,3,
                6,1,4,1,3,3,6,1,4,6,
                6,3,5,7,3,8,5,4,7,8,
                4,1,6,7,5,2,4,6,4,5,
                2,1,7,6,8,4,1,7,2,1,
                6,8,8,2,8,8,1,1,3,4,
                4,8,4,6,8,4,8,5,5,4,
                5,2,8,3,7,5,1,5,2,6,
            ]
        });
    }
}
