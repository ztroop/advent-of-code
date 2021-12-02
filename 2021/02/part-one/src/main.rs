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
    forward: u32,
    up: u32,
    down: u32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            forward: 0,
            up: 0,
            down: 0,
        }
    }

    fn move_forward(&mut self, x: u32) {
        self.forward += x
    }

    fn move_up(&mut self, x: u32) {
        self.up += x
    }

    fn move_down(&mut self, x: u32) {
        self.down += x
    }

    fn get_position(&self) -> u32 {
        self.forward * (self.down - self.up)
    }
}

fn main() {
    let mut submarine = Submarine::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines.flatten() {
            let mut item = line.split_whitespace();
            let direction = item.next().unwrap().trim();
            let distance = item.next().unwrap().trim().parse::<u32>().unwrap();

            match direction {
                "forward" => submarine.move_forward(distance),
                "up" => submarine.move_up(distance),
                "down" => submarine.move_down(distance),
                _ => panic!("Woah! Where are we going?!"),
            }
        }
    }

    println!("The answer is: {}", submarine.get_position())
}
