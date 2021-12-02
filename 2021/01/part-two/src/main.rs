use std::fs;

fn main() {
    let input = fs::read_to_string("./input").unwrap();

    let mut sliding_window: Vec<u32> = Vec::new();
    let mut counter: u32 = 0;

    let measurements = input
        .split('\n')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for i in 0..measurements.len() {
        if i >= 2 {
            sliding_window.push(measurements[i] + measurements[i - 1] + measurements[i - 2]);
        }
    }

    for i in 0..sliding_window.len() {
        if i != 0 && sliding_window[i] > sliding_window[i - 1] {
            counter += 1;
        }
    }

    println!("The answer is: {}", counter)
}
