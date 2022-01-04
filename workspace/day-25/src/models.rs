use std::collections::{HashSet};

pub struct SeaCucumbers {
    east_sea_cucumbers: HashSet<(usize, usize)>,
    south_sea_cucumbers: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl SeaCucumbers {
    pub fn parse_string(content: String) -> SeaCucumbers {
        let lines = content.lines().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();

        let mut east_sea_cucumbers = HashSet::new();
        let mut south_sea_cucumbers = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => { east_sea_cucumbers.insert((x, y)); },
                    'v' => { south_sea_cucumbers.insert((x, y)); },
                    _ => {}
                };
            }
        }

        SeaCucumbers {
            east_sea_cucumbers,
            south_sea_cucumbers,
            width,
            height
        }
    }

    pub fn part_1_stable_sea_cucumbers(&self) -> usize {
        let mut east_sea_cucumbers = self.east_sea_cucumbers.clone();
        let mut south_sea_cucumbers = self.south_sea_cucumbers.clone();

        let mut step = 0;
        loop {
            let mut sea_cucumber_moved = false;

            let mut new_east_sea_cucumbers = HashSet::with_capacity(east_sea_cucumbers.len());
            let mut new_south_sea_cucumbers = HashSet::with_capacity(south_sea_cucumbers.len());

            for &position in &east_sea_cucumbers {
                let new_position = ((position.0 + 1) % self.width, position.1);
                if east_sea_cucumbers.contains(&new_position) || south_sea_cucumbers.contains(&new_position) {
                    new_east_sea_cucumbers.insert(position);
                }
                else {
                    new_east_sea_cucumbers.insert(new_position);
                    sea_cucumber_moved = true;
                }
            }

            for &position in &south_sea_cucumbers {
                let new_position = (position.0, (position.1 + 1) % self.height);
                if new_east_sea_cucumbers.contains(&new_position) || south_sea_cucumbers.contains(&new_position) {
                    new_south_sea_cucumbers.insert(position);
                }
                else {
                    new_south_sea_cucumbers.insert(new_position);
                    sea_cucumber_moved = true;
                }
            }

            east_sea_cucumbers = new_east_sea_cucumbers;
            south_sea_cucumbers = new_south_sea_cucumbers;

            step += 1;

            if !sea_cucumber_moved {
                break;
            }
        }

        step
    }
}

#[cfg(test)]
mod tests {
    use crate::SeaCucumbers;

    #[test]
    fn example_case() {
        let input = SeaCucumbers::parse_string("v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
".to_string());

        assert_eq!(input.part_1_stable_sea_cucumbers(), 58);
    }
}