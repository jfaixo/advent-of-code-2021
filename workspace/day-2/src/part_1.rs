use crate::models::{Input, SubmarineCommand};

pub fn apply_commands(input: &Input) -> i32 {
    let mut current_x = 0;
    let mut current_y = 0;

    for command in &input.commands {
        match command {
            SubmarineCommand::Forward(value) => { current_x += value }
            SubmarineCommand::Down(value) => { current_y += value }
            SubmarineCommand::Up(value) => { current_y -= value }
        }
    }

    current_x * current_y
}

#[cfg(test)]
mod tests {
    use crate::models::{Input};
    use crate::models::SubmarineCommand::{Down, Forward, Up};
    use crate::part_1::apply_commands;

    #[test]
    fn part_1_example_case() {
        let input = Input {
            commands: vec![
                Forward(5),
                Down(5),
                Forward(8),
                Up(3),
                Down(8),
                Forward(2),
            ]
        };

        assert_eq!(150, apply_commands(&input));
    }
}
