use crate::models::Input;

pub fn diagnostic_report(input: &Input) -> u32 {
    oxygen_generator_rating(input) * co2_scrubber_rating(input)
}

fn oxygen_generator_rating(input: &Input) -> u32 {
    let mut values = input.data.clone();

    let mut current_bit_position = input.bit_count as i32 - 1;
    while values.len() > 1 {
        // Compute the bit count at that position
        let total_count = values.len();
        let set_count : usize = values.iter().map(|&value| if (value & 0b1 << current_bit_position) != 0 { 1 } else { 0 }).sum();
        // Remove all non matching values
        values.retain(|&value| if set_count >= total_count - set_count {
            (value & 0b1 << current_bit_position) != 0
        } else {
            (value & 0b1 << current_bit_position) == 0
        });

        current_bit_position -= 1;
    }

    values[0] as u32
}

fn co2_scrubber_rating(input: &Input) -> u32 {
    let mut values = input.data.clone();

    let mut current_bit_position = input.bit_count as i32 - 1;
    while values.len() > 1 {
        // Compute the bit count at that position
        let total_count = values.len();
        let set_count : usize = values.iter().map(|&value| if (value & 0b1 << current_bit_position) != 0 { 1 } else { 0 }).sum();
        // Remove all non matching values
        values.retain(|&value| if set_count < total_count - set_count {
            (value & 0b1 << current_bit_position) != 0
        } else {
            (value & 0b1 << current_bit_position) == 0
        });

        current_bit_position -= 1;
    }

    values[0] as u32
}



#[cfg(test)]
mod tests {
    use crate::models::{Input};
    use crate::part_2::{co2_scrubber_rating, oxygen_generator_rating};

    #[test]
    fn part_2_oxygen_generator_rating_example_case() {
        let input = Input {
            bit_count: 5,
            data: vec![
                0b00100,
                0b11110,
                0b10110,
                0b10111,
                0b10101,
                0b01111,
                0b00111,
                0b11100,
                0b10000,
                0b11001,
                0b00010,
                0b01010,
            ]
        };

        assert_eq!(23, oxygen_generator_rating(&input));
    }

    #[test]
    fn part_2_co2_scrubber_rating_example_case() {
        let input = Input {
            bit_count: 5,
            data: vec![
                0b00100,
                0b11110,
                0b10110,
                0b10111,
                0b10101,
                0b01111,
                0b00111,
                0b11100,
                0b10000,
                0b11001,
                0b00010,
                0b01010,
            ]
        };

        assert_eq!(10, co2_scrubber_rating(&input));
    }
}
