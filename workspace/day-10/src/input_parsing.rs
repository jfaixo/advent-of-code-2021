use std::error::Error;
use std::fs;
use crate::models::{Input, ParsedLine};

pub fn parse_file(file_path: String) -> Result<Input, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
    let parsed_lines = content.lines()
        .map(|line|{
            ParsedLine::from_str(line)
        })
        .collect();

    Ok(Input {
        parsed_lines
    })
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::models::{Input, ParsedLine, Symbol};

    #[test]
    fn parse_example_case() {
        let content = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
".to_string();

        let input = parse_string(content).unwrap();

        assert_eq!(input, Input {
            parsed_lines: vec![
                ParsedLine::Incomplete,
                ParsedLine::Incomplete,
                ParsedLine::Corrupted(Symbol::CurlyBracketClose),
                ParsedLine::Incomplete,
                ParsedLine::Corrupted(Symbol::RoundBracketClose),
                ParsedLine::Corrupted(Symbol::SquareBracketClose),
                ParsedLine::Incomplete,
                ParsedLine::Corrupted(Symbol::RoundBracketClose),
                ParsedLine::Corrupted(Symbol::AngleBracketClose),
                ParsedLine::Incomplete,

            ]
        });
    }
}
