use crate::models::{Board, Input};
use std::error::Error;
use std::fs;

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let lines: Vec<&str> = content.lines().collect();

    let draw_numbers = lines[0]
        .split(',')
        .map(|number| number.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()?;

    let board_count = (lines.len() - 2) / 6 + 1;
    let mut boards = Vec::new();
    let mut numbers: [u8; 25] = [0; 25];
    for n in 0..board_count {
        for i in 0..5 {
            // Parse the line
            let ref mut line_numbers = lines[2 + n * 6 + i]
                .split_ascii_whitespace()
                .map(|number| number.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>()?;
            // Inject it inside the array
            numbers[i * 5..i * 5 + 5].copy_from_slice(line_numbers);
        }
        boards.push(Board::new(numbers));
    }

    Ok(Input {
        draw_numbers,
        boards,
    })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{Board, Input};

    #[test]
    fn parse_example_case() {
        let content = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
        .to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(
            input,
            Input {
                draw_numbers: vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                boards: vec![
                    Board::new([
                        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1,
                        12, 20, 15, 19
                    ]),
                    Board::new([
                        3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14,
                        21, 16, 12, 6,
                    ]),
                    Board::new([
                        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5,
                        2, 0, 12, 3, 7,
                    ])
                ]
            }
        );
    }
}
