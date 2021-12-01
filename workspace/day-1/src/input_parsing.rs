use std::fs;
use crate::models::Input;

pub fn parse_file(file_path: String) -> Input {
    let file_content = fs::read_to_string(file_path)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

fn parse_string(content: String) -> Input {
    let data = content.split_ascii_whitespace().map(|line| line.parse().unwrap()).collect();

    Input {
        data
    }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::Input;

    #[test]
    fn parse_example_case() {
        let content = "199
200
208
210
200
207
240
269
260
263".to_string();

        let input = parse_string(content);

        assert_eq!(input, Input {
            data: vec![
                199,
                200,
                208,
                210,
                200,
                207,
                240,
                269,
                260,
                263,
            ]
        });
    }
}
