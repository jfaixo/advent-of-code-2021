use crate::models::Input;

pub fn simulate_lanternfish_population(input: &Input, days: usize) -> u64 {
    let mut fish_population = input.fish_per_timer.clone();
    for _i in 0..days {
        let fishes_day_0 = fish_population[0];
        fish_population.copy_within(1..9, 0);
        // Reset fishes that gave born
        fish_population[6] += fishes_day_0;
        // Create new fish
        fish_population[8] = fishes_day_0;
    }

    fish_population.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::models::Input;
    use crate::simulate::simulate_lanternfish_population;

    #[test]
    fn part_1_example_case() {
        let input = Input {
            fish_per_timer: [0, 1, 1, 2, 1, 0, 0, 0, 0]
        };

        assert_eq!(5934, simulate_lanternfish_population(&input, 80));
    }

    #[test]
    fn part_2_example_case() {
        let input = Input {
            fish_per_timer: [0, 1, 1, 2, 1, 0, 0, 0, 0]
        };

        assert_eq!(26984457539, simulate_lanternfish_population(&input, 256));
    }
}
