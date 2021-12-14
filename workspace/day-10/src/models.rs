#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub parsed_lines: Vec<ParsedLine>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParsedLine {
    Corrupted(Symbol),
    Incomplete(Vec<Symbol>),
    Valid,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
    RoundBracketOpen,
    RoundBracketClose,
    SquareBracketOpen,
    SquareBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    AngleBracketOpen,
    AngleBracketClose,
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '(' => Symbol::RoundBracketOpen,
            ')' => Symbol::RoundBracketClose,
            '[' => Symbol::SquareBracketOpen,
            ']' => Symbol::SquareBracketClose,
            '{' => Symbol::CurlyBracketOpen,
            '}' => Symbol::CurlyBracketClose,
            '<' => Symbol::AngleBracketOpen,
            '>' => Symbol::AngleBracketClose,
            _ => panic!("Invalid character found"),
        }
    }

    fn opening_symbol(&self) -> Self {
        match self {
            Symbol::RoundBracketClose => Symbol::RoundBracketOpen,
            Symbol::SquareBracketClose => Symbol::SquareBracketOpen,
            Symbol::CurlyBracketClose => Symbol::CurlyBracketOpen,
            Symbol::AngleBracketClose => Symbol::AngleBracketOpen,
            _ => panic!("Tried to match an opening symbol"),
        }
    }
}

impl ParsedLine {
    pub fn from_str(line: &str) -> Self {
        let mut symbols_queue = Vec::new();

        for (index, c) in line.chars().enumerate() {
            match Symbol::from_char(c) {
                symbol
                    if symbol == Symbol::RoundBracketOpen
                        || symbol == Symbol::SquareBracketOpen
                        || symbol == Symbol::CurlyBracketOpen
                        || symbol == Symbol::AngleBracketOpen =>
                {
                    symbols_queue.push(symbol);
                }
                symbol => match symbols_queue.pop() {
                    None => return ParsedLine::Corrupted(symbol),
                    Some(previous_symbol) => {
                        if previous_symbol != symbol.opening_symbol() {
                            return ParsedLine::Corrupted(symbol);
                        }
                    }
                },
            }
        }

        if symbols_queue.len() > 0 {
            ParsedLine::Incomplete(symbols_queue)
        } else {
            ParsedLine::Valid
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ParsedLine, Symbol};

    #[test]
    fn corrupted_chunks() {
        // Corrupted examples
        assert_eq!(
            ParsedLine::from_str("(]"),
            ParsedLine::Corrupted(Symbol::SquareBracketClose)
        );
        assert_eq!(
            ParsedLine::from_str("{()()()>"),
            ParsedLine::Corrupted(Symbol::AngleBracketClose)
        );
        assert_eq!(
            ParsedLine::from_str("(((()))}"),
            ParsedLine::Corrupted(Symbol::CurlyBracketClose)
        );
        assert_eq!(
            ParsedLine::from_str("<([]){()}[{}])"),
            ParsedLine::Corrupted(Symbol::RoundBracketClose)
        );
    }
}
