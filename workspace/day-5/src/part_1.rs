use std::collections::HashMap;
use crate::models::{Input, Point};

pub fn overlapping_points_count(input: &Input) -> usize {

    // Store all points that have a hit in the map in an hashmap, along with their number of crossing lines
    let mut points_in_map : HashMap<Point, usize> = HashMap::new();

    // Process all lines
    for line in &input.lines {
        // Only horizontal or vertical lines
        if line.is_horizontal() || line.is_vertical() {
            // Compute steps to go from a to b
            let mut step_x = line.b.x - line.a.x;
            if step_x != 0 {
                step_x = step_x / step_x.abs();
            }
            let mut step_y = line.b.y - line.a.y;
            if step_y != 0 {
                step_y = step_y / step_y.abs();
            }

            // Walk along the line and insert the points
            let mut current_x = line.a.x;
            let mut current_y = line.a.y;
            while current_x - step_x != line.b.x || current_y - step_y != line.b.y {
                let point = Point { x: current_x, y: current_y };
                if points_in_map.contains_key(&point) {
                    match points_in_map.get_mut(&point) {
                        None => {}
                        Some(count) => { *count += 1; }
                    }
                }
                else {
                    points_in_map.insert(point, 1);
                }

                current_x += step_x;
                current_y += step_y;
            }
        }
    }

    // Compute the score
    let total = points_in_map.iter().map(|(_, count)| if *count > 1 { 1 } else { 0 }).sum();

    total
}

#[cfg(test)]
mod tests {
    use crate::models::{Input, Line, Point};
    use crate::part_1::overlapping_points_count;

    #[test]
    fn part_1_example_case() {
        let input = Input {
            lines: vec![
                Line { a: Point { x: 0, y: 9 }, b: Point { x: 5, y: 9 } },
                Line { a: Point { x: 8, y: 0 }, b: Point { x: 0, y: 8 } },
                Line { a: Point { x: 9, y: 4 }, b: Point { x: 3, y: 4 } },
                Line { a: Point { x: 2, y: 2 }, b: Point { x: 2, y: 1 } },
                Line { a: Point { x: 7, y: 0 }, b: Point { x: 7, y: 4 } },
                Line { a: Point { x: 6, y: 4 }, b: Point { x: 2, y: 0 } },
                Line { a: Point { x: 0, y: 9 }, b: Point { x: 2, y: 9 } },
                Line { a: Point { x: 3, y: 4 }, b: Point { x: 1, y: 4 } },
                Line { a: Point { x: 0, y: 0 }, b: Point { x: 8, y: 8 } },
                Line { a: Point { x: 5, y: 5 }, b: Point { x: 8, y: 2 } },
            ]
        };

        assert_eq!(5, overlapping_points_count(&input));
    }
}
