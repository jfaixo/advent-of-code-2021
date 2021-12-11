use crate::models::{Input, ParsedLine, Symbol};

pub fn autocomplete_score(input: &Input) -> u64 {
    let mut scores = Vec::new();

    for line in &input.parsed_lines {
        match line {
            ParsedLine::Incomplete(symbols) => {
                let mut score = 0;
                for i in (0..symbols.len()).rev() {
                    score *= 5;
                    match symbols[i] {
                        Symbol::RoundBracketOpen => score += 1,
                        Symbol::SquareBracketOpen => score += 2,
                        Symbol::CurlyBracketOpen => score += 3,
                        Symbol::AngleBracketOpen => score += 4,
                        _ => panic!("Invalid opening symbol")
                    }
                }
                scores.push(score);
            }
            _ => {}
        }
    }

    scores.sort();

    scores[scores.len() / 2]
}
