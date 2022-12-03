use crate::enums::{Outcome, Shape};

pub fn letter_to_enums(letters: (&str, &str)) -> (Shape, Shape) {
    let opponent = match letters.0 {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Unexpected letters: {}", letters.0),
    };
    let protagonist = match letters.1 {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Unexpected letters: {}", letters.1),
    };
    (opponent, protagonist)
}

pub fn round_score(round: (Shape, Shape)) -> usize {
    let opponent = round.0;
    let protagonist = round.1;

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
