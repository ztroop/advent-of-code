use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn answer() -> u32 {
    let mut last_measurement: u32 = 0;
    let mut counter: u32 = 0;

    if let Ok(lines) = read_lines("./input.file") {
        for line in lines.flatten() {
            let measurement = line.trim().parse::<u32>().unwrap();
            if last_measurement != 0 && measurement > last_measurement {
                counter += 1
            }
            last_measurement = measurement
        }
    }

    counter
}
