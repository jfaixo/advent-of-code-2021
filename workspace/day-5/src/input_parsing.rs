use crate::models::{Input, Line, Point};
use std::error::Error;
use std::fs;

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path).expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let lines = content
        .lines()
        .map(|string_line| {
            let parts = string_line
                .replace(" -> ", ",")
                .split(',')
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Line {
                a: Point {
                    x: parts[0],
                    y: parts[1],
                },
                b: Point {
                    x: parts[2],
                    y: parts[3],
                },
            }
        })
        .collect();

    Ok(Input { lines })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{Input, Line, Point};

    #[test]
    fn parse_example_case() {
        let content = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"
        .to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(
            input,
            Input {
                lines: vec![
                    Line {
                        a: Point { x: 0, y: 9 },
                        b: Point { x: 5, y: 9 }
                    },
                    Line {
                        a: Point { x: 8, y: 0 },
                        b: Point { x: 0, y: 8 }
                    },
                    Line {
                        a: Point { x: 9, y: 4 },
                        b: Point { x: 3, y: 4 }
                    },
                    Line {
                        a: Point { x: 2, y: 2 },
                        b: Point { x: 2, y: 1 }
                    },
                    Line {
                        a: Point { x: 7, y: 0 },
                        b: Point { x: 7, y: 4 }
                    },
                    Line {
                        a: Point { x: 6, y: 4 },
                        b: Point { x: 2, y: 0 }
                    },
                    Line {
                        a: Point { x: 0, y: 9 },
                        b: Point { x: 2, y: 9 }
                    },
                    Line {
                        a: Point { x: 3, y: 4 },
                        b: Point { x: 1, y: 4 }
                    },
                    Line {
                        a: Point { x: 0, y: 0 },
                        b: Point { x: 8, y: 8 }
                    },
                    Line {
                        a: Point { x: 5, y: 5 },
                        b: Point { x: 8, y: 2 }
                    },
                ]
            }
        );
    }
}
