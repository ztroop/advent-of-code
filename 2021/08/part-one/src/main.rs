use itertools::Itertools;

#[derive(Debug)]
struct DigitDisplay<'a> {
    combinations: Vec<&'a str>,
    digits: Vec<&'a str>,
}

impl From<&'static str> for DigitDisplay<'_> {
    fn from(s: &'static str) -> Self {
        let (left, right) = s.split(" | ").collect_tuple().unwrap();
        let combinations = left.split_whitespace().collect();
        let digits = right.split_whitespace().collect();

        Self {
            combinations,
            digits,
        }
    }
}

fn part_one(data: &'static str) -> usize {
    data.lines()
        .map(DigitDisplay::from)
        .flat_map(|x| x.digits)
        .filter(|d| matches!(d.len(), 2 | 3 | 4 | 7))
        .count()
}

fn main() {
    let data = include_str!("../input");
    let one = part_one(data);

    println!("The first answer is: {:?}", one);
}
