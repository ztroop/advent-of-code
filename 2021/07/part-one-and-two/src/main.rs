use std::cmp::min;

fn parse_input(s: &str) -> Vec<i64> {
    s.split(',').map(|x| x.parse::<i64>().unwrap()).collect()
}

fn part_one(d: &str) -> i64 {
    let mut positions = parse_input(d);
    positions.sort_unstable();
    let center = positions[positions.len() / 2];
    let moves: i64 = positions.iter().map(|x| (x - center).abs()).sum();

    moves
}

fn part_two(d: &str) -> i64 {
    let positions = parse_input(d);

    let lowest = positions.iter().min().unwrap();
    let hightest = positions.iter().max().unwrap();

    let mut minimum_fuel = i64::MAX;
    for i in *lowest..*hightest {
        let mut fuel_needed = 0;
        for p in &positions {
            if *p > i {
                fuel_needed += (0..=*p-i).sum::<i64>();
            } else {
                fuel_needed += (0..=i-*p).sum::<i64>();
            }
        }
        minimum_fuel = min(minimum_fuel, fuel_needed)
    }

    minimum_fuel
}

fn main() {
    let data = include_str!("../input");

    let one = part_one(data);
    let second = part_two(data);

    println!("The first answer is: {:?}", one);
    println!("The second answer is: {:?}", second);
}
