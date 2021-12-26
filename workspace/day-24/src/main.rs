use std::collections::HashMap;

/// w is the input number
/// a is the value from line 5
/// b is the value from line 6
/// c is the value from line 16
///
/// This function is a high level version of the assembly code
fn check_digit(w: i64, a: i64, b: i64, c: i64, previous_z: i64) -> i64 {
    if previous_z % 26 + b != w { // x = 1
        26 * previous_z / a + w + c
    }
    else { // x = 0
        previous_z / a
    }
}

// fn monad(digits: [i64; 14]) -> bool {
//     // The list of parameters a, b and c
//     let parameters = [
//         [1, 11, 6],
//         [1, 11, 14],
//         [1, 15, 13],
//         [26, -14, 1],
//         [1, 10, 6],
//         [26, 0, 13],
//         [26, -6, 6],
//         [1, 13, 3],
//         [26, -3, 8],
//         [1, 13, 14],
//         [1, 15, 4],
//         [26, -2, 7],
//         [26, -9, 15],
//         [26, -2, 1],
//     ];
//
//     let mut pz = 0;
//     for i in 0..14 {
//         pz = check_digit(digits[i], parameters[i][0], parameters[i][1], parameters[i][2], pz);
//     }
//
//     pz == 0
// }

struct Solver {
    intermediate_steps: HashMap<(usize, i64), i64>,
    parameters: [[i64; 3]; 14],
}

impl Solver {
    fn maximize(&mut self, depth: usize, pz: i64) -> i64 {
        if depth == 14 {
            if pz == 0 {
                0
            }
            else {
                -1
            }
        }
        else if let Some(n) = self.intermediate_steps.get(&(depth, pz)) {
            *n
        }
        else {
            let mut best_n = -1;
            for digit in (1..=9).rev() {
                let z = check_digit(digit, self.parameters[depth][0], self.parameters[depth][1], self.parameters[depth][2], pz);
                let n = self.maximize(depth + 1, z);

                if n >= 0 {
                    best_n = 10i64.pow(13 - depth as u32) * digit + n;
                    break;
                }
            }
            self.intermediate_steps.insert((depth, pz), best_n);
            best_n
        }
    }

    fn minimize(&mut self, depth: usize, pz: i64) -> i64 {
        if depth == 14 {
            if pz == 0 {
                0
            }
            else {
                -1
            }
        }
        else if let Some(n) = self.intermediate_steps.get(&(depth, pz)) {
            *n
        }
        else {
            let mut best_n = -1;
            for digit in 1..=9 {
                let z = check_digit(digit, self.parameters[depth][0], self.parameters[depth][1], self.parameters[depth][2], pz);
                let n = self.minimize(depth + 1, z);

                if n >= 0 {
                    best_n = 10i64.pow(13 - depth as u32) * digit + n;
                    break;
                }
            }
            self.intermediate_steps.insert((depth, pz), best_n);
            best_n
        }
    }
}

fn main() {
    let mut solver = Solver {
        intermediate_steps: Default::default(),
        parameters: [
            [1, 11, 6],
            [1, 11, 14],
            [1, 15, 13],
            [26, -14, 1],
            [1, 10, 6],
            [26, 0, 13],
            [26, -6, 6],
            [1, 13, 3],
            [26, -3, 8],
            [1, 13, 14],
            [1, 15, 4],
            [26, -2, 7],
            [26, -9, 15],
            [26, -2, 1],
        ],
    };

    println!("part1: {}", solver.maximize(0, 0));
    solver.intermediate_steps.clear();
    println!("part2: {}", solver.minimize(0, 0));
}
