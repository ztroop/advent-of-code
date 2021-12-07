use std::collections::HashMap;

use itertools::{iterate, Itertools};

fn parse_input(data: &'static str) -> HashMap<i8, usize> {
    data.trim().split(',').map(|v| v.parse().unwrap()).counts()
}

fn fish_generation_sum(data: HashMap<i8, usize>, generations: usize) -> usize {
    fn state(fish: &HashMap<i8, usize>) -> HashMap<i8, usize> {
        // Iterate over the fish and deincrement their internal timer.
        let mut fish: HashMap<_, _> = fish.iter().map(|(&age, &count)| (age - 1, count)).collect();
        // Reset birth timer and create a new fish with timer of 8.
        if let Some(birth) = fish.remove(&-1) {
            *fish.entry(6).or_insert(0) += birth;
            fish.insert(8, birth);
        }
        fish
    }

    iterate(data, state)
        .nth(generations)
        .unwrap()
        .values()
        .sum()
}

fn main() {
    let data = parse_input(include_str!("../input"));
    println!("The answer is {:?}", fish_generation_sum(data, 80)) // 256
}
