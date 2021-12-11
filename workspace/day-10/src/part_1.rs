pub use crate::models::{Input, ParsedLine, Symbol};

pub fn syntax_error_score(input: &Input) -> u64 {
    let mut score = 0;

    for line in &input.parsed_lines {
        match line {
            ParsedLine::Corrupted(symbol) => {
                match symbol {
                    Symbol::RoundBracketClose => score += 3,
                    Symbol::SquareBracketClose => score += 57,
                    Symbol::CurlyBracketClose => score += 1197,
                    Symbol::AngleBracketClose => score += 25137,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use crate::models::{Input, ParsedLine, Symbol};
    use crate::part_1::syntax_error_score;

    #[test]
    fn part_1_example_case() {
        let input = Input {
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
        };

        assert_eq!(syntax_error_score(&input), 26397);
    }
}
