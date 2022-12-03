mod enums;
mod one;
mod two;

fn main() {
    const DATA: &str = include_str!("../input.file");

    let p1_answer: usize = DATA
        .lines()
        .map(|a| one::letter_to_enums(a.split_once(" ").unwrap()))
        .map(one::round_score)
        .sum();

    println!("Part one answer is: {:?}", p1_answer);

    let p2_answer: usize = DATA
        .lines()
        .map(|a| two::letter_to_enums(a.split_once(" ").unwrap()))
        .map(two::round_score)
        .sum();

    println!("Part two answer is: {:?}", p2_answer)
}
