use crate::models::Input;

pub fn all_octopus_flash(input: &Input) -> usize {
    let mut octopuses = input.octopuses;

    let mut step = 0;
    loop {
        let mut flash_count = 0;

        // First increase all by 1
        for i in 0..octopuses.len() {
            octopuses[i] += 1;
        }

        // Then, propagate and count flashes
        loop {
            let mut new_propagation_required = false;
            for y in 0..10 {
                for x in 0..10 {
                    if octopuses[y * 10 + x] > 9 {
                        // Flash !
                        flash_count += 1;
                        octopuses[y * 10 + x] = -1;

                        // Propagate
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if !(dx == 0 && dy == 0)
                                    && (x as i32 + dx) >= 0
                                    && (y as i32 + dy) >= 0
                                    && (x as i32 + dx) < 10
                                    && (y as i32 + dy) < 10
                                {
                                    let x0 = (x as i32 + dx) as usize;
                                    let y0 = (y as i32 + dy) as usize;
                                    let value = octopuses[y0 * 10 + x0];

                                    if value != -1 {
                                        octopuses[y0 * 10 + x0] = value + 1;
                                        if value + 1 > 9 {
                                            new_propagation_required = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !new_propagation_required {
                break;
            }
        }

        // End of propagation, put the fhased octopuses to the 0 state
        for i in 0..octopuses.len() {
            if octopuses[i] == -1 {
                octopuses[i] = 0;
            }
        }

        step += 1;

        if flash_count == 100 {
            break;
        }
    }

    step
}

#[cfg(test)]
mod tests {
    use crate::models::Input;
    use crate::part_2::all_octopus_flash;

    #[test]
    fn part_2_example_case() {
        let input = Input {
            octopuses: [
                5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1,
                7, 3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2,
                4, 6, 4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6,
                8, 4, 8, 5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
            ],
        };

        assert_eq!(all_octopus_flash(&input), 195);
    }
}
