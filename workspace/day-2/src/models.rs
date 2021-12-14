#[derive(Debug, Eq, PartialEq)]
pub enum SubmarineCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Input {
    pub commands: Vec<SubmarineCommand>,
}
