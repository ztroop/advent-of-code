use std::fs::File;
use std::io::{self, BufRead};
use std::panic;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Submarine {
    horizonal_position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            horizonal_position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn move_forward(&mut self, x: usize) {
        self.horizonal_position += x;
        self.depth += self.aim * x;
    }

    fn move_up(&mut self, x: usize) {
        self.aim -= x
    }

    fn move_down(&mut self, x: usize) {
        self.aim += x
    }

    fn get_position(self) -> usize {
        self.horizonal_position * self.depth
    }
}

fn main() {
    let mut submarine = Submarine::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines.flatten() {
            let mut item = line.split_whitespace();
            let direction = item.next().unwrap().trim();
            let distance = item.next().unwrap().trim().parse::<usize>().unwrap();

            match direction {
                "forward" => submarine.move_forward(distance),
                "up" => submarine.move_up(distance),
                "down" => submarine.move_down(distance),
                _ => panic!("Woah! Where are we going?!"),
            }
        }
    }

    println!("The answer is: {:?}", submarine.get_position())
}
