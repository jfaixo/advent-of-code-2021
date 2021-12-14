use std::collections::{HashSet, VecDeque};
use std::error::Error;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct TransparentSheet {
    pub points: HashSet<Point>,
    pub foldings: VecDeque<Folding>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Folding {
    Up(i32),
    Left(i32),
}

impl TransparentSheet {
    pub fn parse_string(content: String) -> Result<TransparentSheet, Box<dyn Error>> {
        let mut transparent_sheet = TransparentSheet::default();

        let mut folding_parsing = false;
        for line in content.lines() {
            if !folding_parsing {
                if line.is_empty() {
                    folding_parsing = true;
                } else {
                    let parts = line
                        .split(',')
                        .map(|n| n.parse::<i32>())
                        .collect::<Result<Vec<_>, _>>()?;
                    transparent_sheet.points.insert(Point {
                        x: parts[0],
                        y: parts[1],
                    });
                }
            } else {
                let parts = line[11..].split('=').collect::<Vec<_>>();
                let value = parts[1].parse::<i32>()?;

                if parts[0] == "x" {
                    transparent_sheet.foldings.push_back(Folding::Left(value));
                } else {
                    transparent_sheet.foldings.push_back(Folding::Up(value));
                }
            }
        }

        Ok(transparent_sheet)
    }

    pub fn count_fold_once(&self) -> usize {
        let mut sheet = self.clone();

        sheet.fold();

        sheet.points.len()
    }

    fn fold(&mut self) -> Result<(), Box<dyn Error>> {
        let fold = self.foldings.pop_front();

        let mut updated_points = HashSet::new();
        match fold {
            Some(Folding::Up(value)) => {
                for point in &self.points {
                    if point.y > value {
                        updated_points.insert(Point {
                            x: point.x,
                            y: value * 2 - point.y,
                        });
                    } else {
                        updated_points.insert(*point);
                    }
                }
                self.points = updated_points;
            }
            Some(Folding::Left(value)) => {
                for point in &self.points {
                    if point.x > value {
                        updated_points.insert(Point {
                            x: value * 2 - point.x,
                            y: point.y,
                        });
                    } else {
                        updated_points.insert(*point);
                    }
                }
                self.points = updated_points;
            }
            None => {}
        }

        Ok(())
    }

    pub fn fold_and_print(&self) {
        let mut sheet = self.clone();
        while sheet.foldings.len() > 0 {
            sheet.fold();
        }

        sheet.print();
    }

    fn print(&self) {
        let mut width = 0;
        let mut height = 0;

        for point in &self.points {
            width = width.max(point.x as usize + 1);
            height = height.max(point.y as usize + 1);
        }
        let mut pixels = vec!['.'; width * height];
        for point in &self.points {
            let index = (point.y as usize * width) + point.x as usize;
            pixels[index] = '#';
        }

        // Display
        for y in 0..height {
            for x in 0..width {
                print!("{}", pixels[y * width + x]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Folding, Point, TransparentSheet};
    use std::collections::{HashSet, VecDeque};

    #[test]
    fn parse_example_case() {
        let content = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"
        .to_string();

        let input = TransparentSheet::parse_string(content).unwrap();

        assert_eq!(
            input,
            TransparentSheet {
                points: HashSet::<_>::from_iter([
                    Point { x: 6, y: 10 },
                    Point { x: 0, y: 14 },
                    Point { x: 9, y: 10 },
                    Point { x: 0, y: 3 },
                    Point { x: 10, y: 4 },
                    Point { x: 4, y: 11 },
                    Point { x: 6, y: 0 },
                    Point { x: 6, y: 12 },
                    Point { x: 4, y: 1 },
                    Point { x: 0, y: 13 },
                    Point { x: 10, y: 12 },
                    Point { x: 3, y: 4 },
                    Point { x: 3, y: 0 },
                    Point { x: 8, y: 4 },
                    Point { x: 1, y: 10 },
                    Point { x: 2, y: 14 },
                    Point { x: 8, y: 10 },
                    Point { x: 9, y: 0 },
                ]),
                foldings: VecDeque::from_iter([Folding::Up(7), Folding::Left(5)])
            }
        );
    }

    #[test]
    fn part_1_example_case() {
        let mut input = TransparentSheet {
            points: HashSet::<_>::from_iter([
                Point { x: 6, y: 10 },
                Point { x: 0, y: 14 },
                Point { x: 9, y: 10 },
                Point { x: 0, y: 3 },
                Point { x: 10, y: 4 },
                Point { x: 4, y: 11 },
                Point { x: 6, y: 0 },
                Point { x: 6, y: 12 },
                Point { x: 4, y: 1 },
                Point { x: 0, y: 13 },
                Point { x: 10, y: 12 },
                Point { x: 3, y: 4 },
                Point { x: 3, y: 0 },
                Point { x: 8, y: 4 },
                Point { x: 1, y: 10 },
                Point { x: 2, y: 14 },
                Point { x: 8, y: 10 },
                Point { x: 9, y: 0 },
            ]),
            foldings: VecDeque::from_iter([Folding::Up(7), Folding::Left(5)]),
        };

        assert_eq!(input.count_fold_once(), 17);

        input.fold_and_print();
    }
}
