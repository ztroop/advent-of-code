use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn scoring(s: char) -> usize {
    match s {
        '(' | ')' => 3,
        '[' | ']' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => 0,
    }
}

fn bust_brackets(chars: Chars) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();
    for c in chars {
        let s = c.to_string();
        if s.is_empty() {
            continue;
        }

        if c == '{' || c == '[' || c == '(' || c == '<' {
            stack.push(c);
            continue;
        }

        if c == '}' || c == ']' || c == ')' || c == '>' {
            if stack.is_empty() {
                continue;
            }

            let last = *stack.last().unwrap();
            if c == '}' && last == '{'
                || c == ')' && last == '('
                || c == ']' && last == '['
                || c == '>' && last == '<'
            {
                stack.pop();
            } else {
                return Some(c);
            }
        }
    }
    None
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let the_score: usize = lines
            .flatten()
            .filter_map(|x| bust_brackets(x.chars()))
            .map(scoring)
            .sum();
        println!("The answer is: {:?}", the_score)
    }
}
