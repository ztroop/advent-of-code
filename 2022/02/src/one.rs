use crate::enums::{Outcome, Shape};

pub fn round_score(round: Vec<(Shape, Outcome)>) -> usize {
    let opponent = round[0].0;
    let protagonist = round[1].0;

    let score = match opponent {
        Shape::Rock => {
            if protagonist == Shape::Paper {
                (Shape::Paper, Outcome::Win)
            } else if protagonist == Shape::Scissors {
                (Shape::Scissors, Outcome::Loss)
            } else {
                (Shape::Rock, Outcome::Draw)
            }
        }
        Shape::Paper => {
            if protagonist == Shape::Scissors {
                (Shape::Scissors, Outcome::Win)
            } else if protagonist == Shape::Rock {
                (Shape::Rock, Outcome::Loss)
            } else {
                (Shape::Paper, Outcome::Draw)
            }
        }
        Shape::Scissors => {
            if protagonist == Shape::Rock {
                (Shape::Rock, Outcome::Win)
            } else if protagonist == Shape::Paper {
                (Shape::Paper, Outcome::Loss)
            } else {
                (Shape::Scissors, Outcome::Draw)
            }
        }
    };
    score.0 as usize + score.1 as usize
}
