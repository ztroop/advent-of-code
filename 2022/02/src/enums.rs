#[derive(PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}
