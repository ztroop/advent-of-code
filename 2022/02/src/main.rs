mod enums;
mod one;
mod two;

const A: &str = "A";
const B: &str = "B";
const C: &str = "C";
const X: &str = "X";
const Y: &str = "Y";
const Z: &str = "Z";

fn letter_to_enums(letter: &str) -> (enums::Shape, enums::Outcome) {
    match letter {
        A | X => (enums::Shape::Rock, enums::Outcome::Loss),
        B | Y => (enums::Shape::Paper, enums::Outcome::Draw),
        C | Z => (enums::Shape::Scissors, enums::Outcome::Win),
        &_ => panic!("Unexpected letter: {}", letter),
    }
}

fn main() {
    const DATA: &str = include_str!("../input.file");

    let letter_mapping = DATA.lines().map(|a| {
        a.split(" ")
            .map(letter_to_enums)
            .collect::<Vec<(enums::Shape, enums::Outcome)>>()
    });

    let p1_answer: usize = letter_mapping.clone().map(one::round_score).sum();

    println!("Part one answer is: {:?}", p1_answer);

    let p2_answer: usize = letter_mapping.clone().map(two::round_score).sum();

    println!("Part two answer is: {:?}", p2_answer)
}
