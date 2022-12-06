fn priority(item: char) -> usize {
    if item.is_ascii_lowercase() {
        item as usize - 'a' as usize + 1
    } else {
        item as usize - 'A' as usize + 27
    }
}

fn part_one(bag: (&str, &str)) -> usize {
    let mut selected = None;

    for item in bag.0.chars() {
        let result = bag.1.chars().any(|x| x == item);
        if result {
            selected = Some(item);
            break;
        }
    }

    if selected.is_some() {
        priority(selected.unwrap())
    } else {
        0
    }
}

fn part_two(bags: &[&str]) -> usize {
    let mut selected = None;

    for item in bags[0].chars() {
        if bags[1].chars().any(|x| x == item) && bags[2].chars().any(|x| x == item) {
            selected = Some(item);
            break;
        }
    }

    if selected.is_some() {
        priority(selected.unwrap())
    } else {
        0
    }
}

fn main() {
    const DATA: &str = include_str!("../input.file");

    let p1_answer: usize = DATA
        .lines()
        .map(|a| part_one(a.split_at(a.len() / 2)))
        .sum();

    println!("Part one answer is: {:?}", p1_answer);

    let p2_answer: usize = DATA
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|a| part_two(a))
        .sum();

    println!("Part two answer is: {:?}", p2_answer);
}
