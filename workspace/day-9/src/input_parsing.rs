use std::error::Error;
use std::fs;
use crate::models::{HeightMap, Input};

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let data = content.lines()
        .map(|line|{
            line.chars().map(|c| { c as u8 - '0' as u8 }).collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    Ok(Input {
        map: HeightMap::new(data)
    })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{HeightMap, Input};

    #[test]
    fn parse_example_case() {
        let content = "2199943210
3987894921
9856789892
8767896789
9899965678
".to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(input, Input {
            map: HeightMap::new(vec![
                vec![2,1,9,9,9,4,3,2,1,0],
                vec![3,9,8,7,8,9,4,9,2,1],
                vec![9,8,5,6,7,8,9,8,9,2],
                vec![8,7,6,7,8,9,6,7,8,9],
                vec![9,8,9,9,9,6,5,6,7,8],
            ])
        });
    }
}
