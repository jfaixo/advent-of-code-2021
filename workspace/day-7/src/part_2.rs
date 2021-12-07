use crate::models::Input;

pub fn cheapest_alignment(input: &Input) -> usize {

    let mut low_bound_position = 0;
    let mut low_value = fuel_cost(input, low_bound_position);

    let mut step = input.crab_positions.len() as i64 / 3;
    let mut current_position = low_bound_position + step;

    loop {
        if current_position < 0 || current_position > input.crab_positions.len() as i64 {
            step = - step / 2;
        }
        else {
            let current_value = fuel_cost(input, current_position);

            if current_value > low_value {
                if step.abs() > 1 {
                    step = - step / 2;
                }
                else {
                    break;
                }
            }

            low_bound_position = current_position;
            low_value = current_value;
        }

        current_position += step;
    }


    fuel_cost(input, low_bound_position)
}

fn fuel_cost(input: &Input, position: i64) -> usize {
    let position = position as usize;
    input.crab_positions.iter().map(|&crab| {
        let diff = if crab < position {
            position - crab
        } else {
            crab - position
        };
        diff * (diff + 1) / 2
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::models::Input;
    use crate::part_2::cheapest_alignment;

    #[test]
    fn part_2_example_case() {
        let input = Input {
            crab_positions: vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16]
        };

        assert_eq!(168, cheapest_alignment(&input));
    }
}
