use crate::models::{Input, SubmarineCommand};

pub fn apply_commands(input: &Input) -> i32 {
    let mut current_x = 0;
    let mut current_y = 0;
    let mut aim = 0;

    for command in &input.commands {
        match command {
            SubmarineCommand::Forward(value) => {
                current_x += value;
                current_y += aim * value
            }
            SubmarineCommand::Down(value) => aim += value,
            SubmarineCommand::Up(value) => aim -= value,
        }
    }

    current_x * current_y
}

#[cfg(test)]
mod tests {
    use crate::models::Input;
    use crate::models::SubmarineCommand::{Down, Forward, Up};
    use crate::part_2::apply_commands;

    #[test]
    fn part_2_example_case() {
        let input = Input {
            commands: vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)],
        };

        assert_eq!(900, apply_commands(&input));
    }
}
