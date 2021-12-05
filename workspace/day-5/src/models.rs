
#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub lines: Vec<Line>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32
}
