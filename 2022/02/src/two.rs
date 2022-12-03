use crate::enums::{Outcome, Shape};

pub fn letter_to_enums(letters: (&str, &str)) -> (Shape, Outcome) {
    let shape = match letters.0 {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unexpected letter: {}", letters.0),
    };
    let outcome = match letters.1 {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unexpected letter: {}", letters.1),
    };
    (shape, outcome)
}

pub fn round_score(round: (Shape, Outcome)) -> usize {
    let opponent = round.0;
    let outcome = &round.1;

    let score = match outcome {
        Outcome::Win => {
            if opponent == Shape::Rock {
                (Shape::Paper, Outcome::Win)
            } else if opponent == Shape::Paper {
                (Shape::Scissors, Outcome::Win)
            } else {
                (Shape::Rock, Outcome::Win)
            }
        }
        Outcome::Draw => {
            if opponent == Shape::Rock {
                (Shape::Rock, Outcome::Draw)
            } else if opponent == Shape::Paper {
                (Shape::Paper, Outcome::Draw)
            } else {
                (Shape::Scissors, Outcome::Draw)
            }
        }
        Outcome::Loss => {
            if opponent == Shape::Rock {
                (Shape::Scissors, Outcome::Loss)
            } else if opponent == Shape::Paper {
                (Shape::Rock, Outcome::Loss)
            } else {
                (Shape::Paper, Outcome::Loss)
            }
        }
    };
    score.0 as usize + score.1 as usize
}
