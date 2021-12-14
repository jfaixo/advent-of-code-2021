#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub draw_numbers: Vec<u8>,
    pub boards: Vec<Board>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
    numbers: [u8; 25],
    board_state: [bool; 25],
    is_won: bool,
}

impl Board {
    pub fn new(numbers: [u8; 25]) -> Self {
        Board {
            numbers,
            board_state: [false; 25],
            is_won: false,
        }
    }

    pub fn set(&mut self, number: u8) -> bool {
        for (index, &board_number) in self.numbers.iter().enumerate() {
            // Check if the number matches a cell of the board
            if number == board_number {
                // It matches !
                self.board_state[index] = true;

                // Check for a winning state
                let col = index % 5;
                let row = (index - col) / 5;

                // Winning Row || Winning Col ?
                self.is_won |= (self.board_state[row * 5 + 0]
                    && self.board_state[row * 5 + 1]
                    && self.board_state[row * 5 + 2]
                    && self.board_state[row * 5 + 3]
                    && self.board_state[row * 5 + 4])
                    || (self.board_state[5 * 0 + col]
                        && self.board_state[5 * 1 + col]
                        && self.board_state[5 * 2 + col]
                        && self.board_state[5 * 3 + col]
                        && self.board_state[5 * 4 + col]);
                return self.is_won;
            }
        }

        // No win
        false
    }

    pub fn compute_score(&self) -> u32 {
        let mut score: u32 = 0;

        for i in 0..25 {
            if self.board_state[i] == false {
                score += self.numbers[i] as u32;
            }
        }

        score
    }

    pub fn is_won(&self) -> bool {
        self.is_won
    }
}
