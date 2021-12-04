use crate::models::{Input};

pub fn find_last_winning_board(input: &Input) -> u32 {
    let ref mut boards = input.boards.clone();

    let mut winning_board_count = 0;
    let mut winning_board_and_number = None;
    'outer:
    for &n in &input.draw_numbers {
        for i in 0..boards.len() {
            if boards[i].is_won() == false && boards[i].set(n) {
                winning_board_count += 1;
                if winning_board_count == boards.len() {
                    winning_board_and_number = Some((boards[i].clone(), n));
                    break 'outer;
                }
            }
        }
    }

    match winning_board_and_number {
        Some((board, n)) => {
            board.compute_score() * n as u32
        }
        None => {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Board, Input};
    use crate::part_2::find_last_winning_board;

    #[test]
    fn part_2_example_case() {
        let input = Input {
            draw_numbers: vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
            boards: vec![
                Board::new([
                    22, 13, 17, 11, 0,
                    8, 2, 23, 4, 24,
                    21, 9, 14, 16, 7,
                    6, 10, 3, 18, 5,
                    1, 12, 20, 15, 19
                ]),
                Board::new([
                    3, 15, 0, 2, 22,
                    9, 18, 13, 17,  5,
                    19, 8, 7, 25, 23,
                    20, 11, 10, 24,  4,
                    14, 21, 16, 12, 6,
                ]),
                Board::new([
                    14, 21, 17, 24,  4,
                    10, 16, 15, 9, 19,
                    18, 8, 23, 26, 20,
                    22, 11, 13, 6,  5,
                    2, 0, 12, 3, 7,
                ])
            ]
        };

        assert_eq!(1924, find_last_winning_board(&input));
    }
}
