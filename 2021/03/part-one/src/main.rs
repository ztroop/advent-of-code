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

#[derive(Debug, Default)]
struct DianosticReport {
    binary_aggregate: [u32; 12],
    line_processed: u32,
}

impl DianosticReport {
    fn process(&mut self, line: String) {
        for (i, v) in line.chars().enumerate() {
            self.binary_aggregate[i] += v.to_digit(10).unwrap();
        }
        self.line_processed += 1
    }

    fn get_power_level(&mut self) -> u32 {
        let rate_scale: u32 = self.line_processed / 2;
        let mut gamma_rate: String = "".to_owned();
        let mut epsilon_rate: String = "".to_owned();

        for i in self.binary_aggregate {
            if i > rate_scale {
                gamma_rate.push_str("1");
                epsilon_rate.push_str("0");
            } else {
                gamma_rate.push_str("0");
                epsilon_rate.push_str("1");
            }
        }

        u32::from_str_radix(&gamma_rate, 2).unwrap() * u32::from_str_radix(&epsilon_rate, 2).unwrap()
    }
}

fn main() {
    let mut report = DianosticReport::default();

    if let Ok(lines) = read_lines("./input") {
        for line in lines.flatten() {
            report.process(line)
        }
    }

    println!("{:?}", report);
    println!("{:?}", report.get_power_level())
}