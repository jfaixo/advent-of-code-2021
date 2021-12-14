use crate::models::Input;

pub fn diagnostic_report(input: &Input) -> u32 {
    let mut set_count = vec![0; input.bit_count];
    for n in input.data.iter() {
        for bit_position in 0..input.bit_count {
            if n & 0b1 << bit_position != 0 {
                set_count[bit_position] += 1;
            }
        }
    }

    let mut bit_mask = 0;
    let mut gamma_rate = 0;
    for bit_position in 0..input.bit_count {
        bit_mask |= 0b1 << bit_position;
        if set_count[bit_position] > input.data.len() - set_count[bit_position] {
            gamma_rate |= 0b1 << bit_position;
        }
    }

    let epsilon_rate = (!gamma_rate) & bit_mask;

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use crate::models::Input;
    use crate::part_1::diagnostic_report;

    #[test]
    fn part_1_example_case() {
        let input = Input {
            bit_count: 5,
            data: vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ],
        };

        assert_eq!(198, diagnostic_report(&input));
    }
}
