use crate::models::{Entry, Input};
use itertools::Itertools;
use std::error::Error;
use std::fs;

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let data = content
        .lines()
        .map(|line| {
            let parts = line.split('|').collect::<Vec<&str>>();

            let patterns = parts[0]
                .trim()
                .split_ascii_whitespace()
                .map(|str| str.chars().sorted().collect::<String>())
                .collect::<Vec<String>>();

            let values = parts[1]
                .trim()
                .split_ascii_whitespace()
                .map(|str| str.chars().sorted().collect::<String>())
                .collect::<Vec<String>>();

            Entry::new(patterns, values)
        })
        .collect();

    Ok(Input { data })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{Entry, Input};

    #[test]
    fn parse_example_case() {
        let content =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
"
            .to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(
            input,
            Input {
                data: vec![Entry::new(
                    vec![
                        "ab".to_string(),
                        "abd".to_string(),
                        "abef".to_string(),
                        "bcdef".to_string(),
                        "acdfg".to_string(),
                        "abcdf".to_string(),
                        "abcdef".to_string(),
                        "bcdefg".to_string(),
                        "abcdeg".to_string(),
                        "abcdefg".to_string()
                    ],
                    vec![
                        "bcdef".to_string(),
                        "abcdf".to_string(),
                        "bcdef".to_string(),
                        "abcdf".to_string(),
                    ]
                )]
            }
        );
    }
}
