
#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub map: HeightMap,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HeightMap {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl HeightMap {
    pub fn new(data: Vec<Vec<u8>>) -> HeightMap {
        let width = data[0].len();
        let height = data.len();
        let data = data.into_iter().flatten().collect();

        HeightMap {
            data,
            width,
            height
        }
    }

    pub fn flow_points(&self) -> Vec<(usize, usize)> {{
        let mut flow_points = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.data[y * self.width + x];

                if (x == 0 || self.data[y * self.width + x - 1] > value) && (x + 1 >= self.width || self.data[y * self.width + x + 1] > value) &&
                    (y == 0 || self.data[(y - 1) * self.width + x] > value) && (y + 1 >= self.height || self.data[(y + 1) * self.width + x] > value) {
                    flow_points.push((x, y));
                }
            }
        }

        flow_points
    }}

    pub fn risk_level(&self) -> usize {
        self.flow_points()
            .iter()
            .map(|(x, y)| self.data[y * self.width + x] as usize + 1)
            .sum()
    }

    pub fn find_basins(&self) -> usize {
        let mut basin_sizes = self.flow_points()
            .iter()
            .map(|(x, y)| { self.basin_size(*x, *y) })
            .collect::<Vec<usize>>();

        basin_sizes.sort();
        basin_sizes.reverse();

        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let ref mut basin_map = vec![false; self.width * self.height];

        self.recursive_basin_size(basin_map, x, y)
    }

    fn recursive_basin_size(&self, basin_map: &mut Vec<bool>, x: usize, y: usize) -> usize {
        let mut count = 0;

        if self.data[y *self.width + x] < 9
        {
            basin_map[y * self.width + x] = true;
            count += 1;

            if x > 0 && !basin_map[y * self.width + x - 1] {
                count += self.recursive_basin_size(basin_map, x - 1, y);
            }
            if x + 1 < self.width && !basin_map[y * self.width + x + 1] {
                count += self.recursive_basin_size(basin_map, x + 1, y);
            }
            if y > 0 && !basin_map[(y - 1) * self.width + x] {
                count += self.recursive_basin_size(basin_map, x, y - 1);
            }
            if y + 1 < self.height && !basin_map[(y + 1) * self.width + x] {
                count += self.recursive_basin_size(basin_map, x, y + 1);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{HeightMap, Input};

    #[test]
    fn count_low_points() {
        let input = Input {
            map: HeightMap::new(vec![
                vec![2,1,9,9,9,4,3,2,1,0],
                vec![3,9,8,7,8,9,4,9,2,1],
                vec![9,8,5,6,7,8,9,8,9,2],
                vec![8,7,6,7,8,9,6,7,8,9],
                vec![9,8,9,9,9,6,5,6,7,8],
            ])
        };

        assert_eq!(15, input.map.risk_level());
    }

    #[test]
    fn find_basins() {
        let input = Input {
            map: HeightMap::new(vec![
                vec![2,1,9,9,9,4,3,2,1,0],
                vec![3,9,8,7,8,9,4,9,2,1],
                vec![9,8,5,6,7,8,9,8,9,2],
                vec![8,7,6,7,8,9,6,7,8,9],
                vec![9,8,9,9,9,6,5,6,7,8],
            ])
        };

        assert_eq!(1134, input.map.find_basins());
    }
}
