use std::collections::{HashSet};
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    pub image_enhancer: ImageEnhancer,
    pub base_image: Image,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImageEnhancer {
    algorithm: Vec<i32>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub data: HashSet<(i32, i32)>,
    pub x_range: [i32; 2],
    pub y_range: [i32; 2],
    pub infinite_value: i32,
}


impl Input {
    pub fn parse_string(content: String) -> Result<Input, Box<dyn Error>> {
        let lines = content.lines().collect::<Vec<_>>();

        let algorithm = lines[0].chars().map(|c| if c == '#' { 1 } else { 0 }).collect();

        let mut data = HashSet::new();
        for y in 2..lines.len() {
            for (x, c) in lines[y].chars().enumerate() {
                if c == '#' {
                    data.insert((x as i32, y as i32 - 2));
                }
            }
        }

        Ok(Input {
            image_enhancer: ImageEnhancer { algorithm },
            base_image: Image {
                data,
                x_range: [-1, lines[2].len() as i32],
                y_range: [-1, lines.len() as i32 - 2],
                infinite_value: 0,
            }
        })
    }

    pub fn multiple_enhance(&self, n: usize) -> usize {
        let mut image = self.base_image.clone();

        for _i in 0..n {
            image = self.image_enhancer.enhance(&image);
        }

        image.data.len()
    }
}

impl ImageEnhancer {

    fn enhance(&self, image: &Image) -> Image {
        let mut result_image = Image {
            data: HashSet::with_capacity(image.data.len()),
            x_range: [image.x_range[0] - 1, image.x_range[1] + 1],
            y_range: [image.y_range[0] - 1, image.y_range[1] + 1],
            infinite_value: if image.infinite_value == 0 { self.algorithm[0] } else { self.algorithm[511] }
        };

        // For each pixel of the image that is not in the infinite space, compute its value
        for y in image.y_range[0]..=image.y_range[1] {
            for x in image.x_range[0]..=image.x_range[1] {
                // Compute the 9 bits
                let index = image.get_pixel(x - 1, y - 1) << 8 | image.get_pixel(x, y - 1) << 7 | image.get_pixel(x + 1, y - 1) << 6 |
                    image.get_pixel(x - 1, y) << 5 | image.get_pixel(x, y) << 4 | image.get_pixel(x + 1, y) << 3 |
                    image.get_pixel(x - 1, y + 1 ) << 2 | image.get_pixel(x, y + 1) << 1 | image.get_pixel(x + 1, y + 1);

                if self.algorithm[index as usize] == 1 {
                    result_image.data.insert((x, y));
                }
            }
        }

        result_image
    }
}

impl Image {
    fn get_pixel(&self, x: i32, y: i32) -> i32 {
        if x <= self.x_range[0] || x >= self.x_range[1] || y <= self.y_range[0] || y >= self.y_range[1] {
            self.infinite_value
        }
        else {
            if self.data.contains(&(x, y)) {
                1
            }
            else {
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::models::Input;

    #[test]
    fn parse_example_case() {
        let input = Input::parse_string("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
".to_string()).unwrap();

        assert_eq!(input.image_enhancer.algorithm[0], 0);
        assert_eq!(input.image_enhancer.algorithm[1], 0);
        assert_eq!(input.image_enhancer.algorithm[2], 1);
        assert_eq!(input.image_enhancer.algorithm[511], 1);

        assert_eq!(input.base_image.data, HashSet::from([
            (0, 0),
            (3, 0),
            (0, 1),
            (0, 2),
            (1, 2),
            (4, 2),
            (2, 3),
            (2, 4),
            (3, 4),
            (4, 4),
        ]));
        assert_eq!(input.base_image.x_range, [-1, 5]);
        assert_eq!(input.base_image.y_range, [-1, 5]);
        assert_eq!(input.base_image.infinite_value, 0);
    }

    #[test]
    fn enhance_1_example_case() {
        let input = Input::parse_string("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
".to_string()).unwrap();

        let image = input.image_enhancer.enhance(&input.base_image);

        assert_eq!(image.x_range, [-2, 6]);
        assert_eq!(image.y_range, [-2, 6]);
        assert_eq!(image.infinite_value, 0);
        assert_eq!(image.data, HashSet::from([
            (0, -1),
            (1, -1),
            (3, -1),
            (4, -1),
            (-1, 0),
            (2, 0),
            (4, 0),
            (-1, 1),
            (0, 1),
            (2, 1),
            (5, 1),
            (-1, 2),
            (0, 2),
            (1, 2),
            (2, 2),
            (5, 2),
            (0, 3),
            (3, 3),
            (4, 3),
            (1, 4),
            (2, 4),
            (5, 4),
            (2, 5),
            (4, 5),
        ]));
    }

    #[test]
    fn part_1_example_case() {
        let input = Input::parse_string("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
".to_string()).unwrap();

        assert_eq!(input.multiple_enhance(2), 35);
    }

    #[test]
    fn part_2_example_case() {
        let input = Input::parse_string("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
".to_string()).unwrap();

        assert_eq!(input.multiple_enhance(50), 3351);
    }
}
