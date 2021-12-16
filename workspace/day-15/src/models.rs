use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ChitonDensityMap {
    map: Vec<usize>,
    width: usize,
    height: usize
}

impl ChitonDensityMap {
    pub fn parse_string(content: String) -> Result<ChitonDensityMap, Box<dyn Error>> {
        let mut density_map = ChitonDensityMap::default();

        let lines = content.lines().collect::<Vec<_>>();
        density_map.width = lines[0].len();
        density_map.height = lines.len();

        density_map.map = lines
            .iter()
            .flat_map(|&line| {
                line
                    .chars()
                    .map(|c| (c as u8 - '0' as u8) as usize)
            }).collect::<Vec<_>>();

        Ok(density_map)
    }

    pub fn shortest_path_score(&self) -> usize {
        let mut costs = vec![usize::MAX; self.width * self.height];
        costs[0] = 0;
        let mut queue = Vec::with_capacity(100);

        queue.push((0, 0));

        while queue.len() > 0 {
            let (x, y) = queue.pop().unwrap();
            let current_cost = costs[y * self.width + x];

            // Left
            if x > 0 {
                let target_cost = costs[y * self.width + x - 1];
                if target_cost == usize::MAX || current_cost + self.map[y * self.width + x - 1] < target_cost {
                    costs[y * self.width + x - 1] = current_cost + self.map[y * self.width + x - 1];
                    queue.push((x - 1, y));
                }
            }

            // Right
            if x < self.width - 1 {
                let target_cost = costs[y * self.width + x + 1];
                if target_cost == usize::MAX || current_cost + self.map[y * self.width + x + 1] < target_cost {
                    costs[y * self.width + x + 1] = current_cost + self.map[y * self.width + x + 1];
                    queue.push((x + 1, y));
                }
            }

            // Top
            if y > 0 {
                let target_cost = costs[(y - 1) * self.width + x];
                if target_cost == usize::MAX || current_cost + self.map[(y - 1) * self.width + x] < target_cost {
                    costs[(y - 1) * self.width + x] = current_cost + self.map[(y - 1) * self.width + x];
                    queue.push((x, y - 1));
                }
            }

            // Bottom
            if y < self.height - 1 {
                let target_cost = costs[(y + 1) * self.width + x];
                if target_cost == usize::MAX || current_cost + self.map[(y + 1) * self.width + x] < target_cost {
                    costs[(y + 1) * self.width + x] = current_cost + self.map[(y + 1) * self.width + x];
                    queue.push((x, y + 1));
                }
            }
        }

        costs[costs.len() - 1]
    }

    pub fn shortest_path_score_5x(&self) -> usize {
        let mut costs = vec![usize::MAX; (5 * self.width) * (5 * self.height)];
        costs[0] = 0;
        let mut queue = BinaryHeap::new();

        queue.push(State {
            x: 0,
            y: 0,
            cost: 0
        });

        while queue.len() > 0 {
            let state = queue.pop().unwrap();
            let x = state.x;
            let y = state.y;
            let current_cost = costs[y * 5 * self.width + x];

            if x == 5 * self.width - 1 && y == 5 * self.height - 1 {
                break;
            }

            // Left
            if x > 0 {
                let target_cost = costs[y * 5 * self.width + x - 1];
                let cell_cost = self.get_5x_cost(x - 1, y);
                if target_cost == usize::MAX || current_cost + cell_cost < target_cost {
                    costs[y * 5 * self.width + x - 1] = current_cost + cell_cost;
                    queue.push(State {
                        x: x - 1,
                        y: y,
                        cost: current_cost + cell_cost
                    });
                }
            }

            // Right
            if x < 5 * self.width - 1 {
                let target_cost = costs[y * 5 * self.width + x + 1];
                let cell_cost = self.get_5x_cost(x + 1, y);
                if target_cost == usize::MAX || current_cost + cell_cost < target_cost {
                    costs[y * 5 * self.width + x + 1] = current_cost + cell_cost;
                    queue.push(State {
                        x: x + 1,
                        y: y,
                        cost: current_cost + cell_cost
                    });
                }
            }

            // Top
            if y > 0 {
                let target_cost = costs[(y - 1) * 5 * self.width + x];
                let cell_cost = self.get_5x_cost(x, y - 1);
                if target_cost == usize::MAX || current_cost + cell_cost < target_cost {
                    costs[(y - 1) * 5 * self.width + x] = current_cost + cell_cost;
                    queue.push(State {
                        x: x,
                        y: y - 1,
                        cost: current_cost + cell_cost
                    });
                }
            }

            // Bottom
            if y <  5 * self.height - 1 {
                let target_cost = costs[(y + 1) * 5 * self.width + x];
                let cell_cost = self.get_5x_cost(x, y + 1);
                if target_cost == usize::MAX || current_cost + cell_cost < target_cost {
                    costs[(y + 1) * 5 * self.width + x] = current_cost + cell_cost;
                    queue.push(State {
                        x: x,
                        y: y + 1,
                        cost: current_cost + cell_cost
                    });
                }
            }
        }

        costs[costs.len() - 1]
    }

    fn get_5x_cost(&self, x: usize, y: usize) -> usize {
        let (xq, xm) = (x / self.width, x % self.width);
        let (yq, ym) = (y / self.height, y % self.height);

        let cost = self.map[ym * self.width + xm] + xq + yq;
        if cost > 9 {
            cost % 10 + 1
        }
        else {
            cost
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.y.cmp(&other.y))
                .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::ChitonDensityMap;

    #[test]
    fn parse_example_case() {
        let content = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"
        .to_string();

        let input = ChitonDensityMap::parse_string(content).unwrap();

        assert_eq!(input, ChitonDensityMap {
            map: vec![
                1,1,6,3,7,5,1,7,4,2,
                1,3,8,1,3,7,3,6,7,2,
                2,1,3,6,5,1,1,3,2,8,
                3,6,9,4,9,3,1,5,6,9,
                7,4,6,3,4,1,7,1,1,1,
                1,3,1,9,1,2,8,1,3,7,
                1,3,5,9,9,1,2,4,2,1,
                3,1,2,5,4,2,1,6,3,9,
                1,2,9,3,1,3,8,5,2,1,
                2,3,1,1,9,4,4,5,8,1,
            ],
            width: 10,
            height: 10
        });
    }

    #[test]
    fn part_1_example_case() {
        let input = ChitonDensityMap {
            map: vec![
                1,1,6,3,7,5,1,7,4,2,
                1,3,8,1,3,7,3,6,7,2,
                2,1,3,6,5,1,1,3,2,8,
                3,6,9,4,9,3,1,5,6,9,
                7,4,6,3,4,1,7,1,1,1,
                1,3,1,9,1,2,8,1,3,7,
                1,3,5,9,9,1,2,4,2,1,
                3,1,2,5,4,2,1,6,3,9,
                1,2,9,3,1,3,8,5,2,1,
                2,3,1,1,9,4,4,5,8,1,
            ],
            width: 10,
            height: 10
        };

        assert_eq!(input.shortest_path_score(), 40);
    }

    #[test]
    fn part_2_example_case() {
        let input = ChitonDensityMap {
            map: vec![
                1,1,6,3,7,5,1,7,4,2,
                1,3,8,1,3,7,3,6,7,2,
                2,1,3,6,5,1,1,3,2,8,
                3,6,9,4,9,3,1,5,6,9,
                7,4,6,3,4,1,7,1,1,1,
                1,3,1,9,1,2,8,1,3,7,
                1,3,5,9,9,1,2,4,2,1,
                3,1,2,5,4,2,1,6,3,9,
                1,2,9,3,1,3,8,5,2,1,
                2,3,1,1,9,4,4,5,8,1,
            ],
            width: 10,
            height: 10
        };

        assert_eq!(input.shortest_path_score_5x(), 315);
    }
}
