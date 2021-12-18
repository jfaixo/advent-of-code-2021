
pub struct TargetArea {
    pub x_range: [i32; 2],
    pub y_range: [i32; 2],
}

struct SimulationState {
    position: [i32; 2],
    velocity: [i32; 2]
}

#[derive(Debug, Eq, PartialEq)]
enum SimulationResult {
    TooLow,
    Hit,
    TooFar
}

impl TargetArea {
    fn simulate(&self, initial_velocity: [i32; 2]) -> (SimulationResult, i32) {
        let mut state = SimulationState {
            position: [0, 0],
            velocity: initial_velocity
        };
        let mut highest_position = 0;

        let drag_effect = - initial_velocity[0].signum();
        loop {
            state.position[0] += state.velocity[0];
            state.position[1] += state.velocity[1];
            if state.velocity[0] != 0 {
                state.velocity[0] += drag_effect;
            }
            state.velocity[1] -= 1;

            if state.position[1] > highest_position {
                highest_position = state.position[1];
            }


            if state.position[1] < self.y_range[0] {
                let distance = if state.position[0] > self.x_range[1] {
                    state.position[0] - self.x_range[1]
                }
                else if state.position[0] < self.x_range[0] {
                    self.x_range[0] - state.position[0]
                }
                else {
                    0
                };
                return (SimulationResult::TooLow, distance);
            }
            else if state.position[0] > self.x_range[1] {
                return (SimulationResult::TooFar, 0);
            }
            else if state.position[0] >= self.x_range[0] && state.position[1] <= self.y_range[1] {
                return (SimulationResult::Hit, highest_position);
            }
        }
    }

    pub fn find_highest_hit(&self) -> i32 {
        let mut best_height = 0;

        for y in -160..160 {
            for x in 0..162 {
                match self.simulate([x, y]) {
                    (SimulationResult::Hit, height) => {
                        if height > best_height {
                            best_height = height
                        }
                    }
                    _ => {}
                }
            }
        }

        best_height
    }

    pub fn count_viable_initial_velocity(&self) -> i32 {
        let mut viable_velocity_count  = 0;

        for y in -160..160 {
            for x in 0..162 {
                match self.simulate([x, y]) {
                    (SimulationResult::Hit, height) => {
                        viable_velocity_count += 1
                    }
                    _ => {}
                }
            }

        }

        viable_velocity_count
    }
}

#[cfg(test)]
mod tests {
    use crate::models::SimulationResult;
    use crate::TargetArea;

    #[test]
    fn example_case() {
        let target = TargetArea {
            x_range: [20, 30],
            y_range: [-10, -5]
        };

        assert_eq!(target.simulate([7, 2]), (SimulationResult::Hit, 3));
        assert_eq!(target.simulate([6, 9]), (SimulationResult::Hit, 45));
    }

    #[test]
    fn search_example_case() {
        let target = TargetArea {
            x_range: [20, 30],
            y_range: [-10, -5]
        };

        assert_eq!(target.find_highest_hit(), 45);
    }

    #[test]
    fn count_example_case() {
        let target = TargetArea {
            x_range: [20, 30],
            y_range: [-10, -5]
        };

        assert_eq!(target.count_viable_initial_velocity(), 112);
    }
}
