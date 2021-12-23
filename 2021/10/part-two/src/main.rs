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

fn score(s: Vec<char>) -> usize {
    let mut total: usize = 0;
    for c in s {
        let score: usize = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
        total = (total * 5) + score
    }
    total
}

fn brackets(chars: Chars) -> Option<Vec<char>> {
    let mut stack: Vec<char> = Vec::new();
    let mut missing_chars: Vec<char> = Vec::new();

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
                return None;
            }
        }
    }

    if stack.is_empty() {
        return None;
    }

    for i in stack.iter().rev() {
        let missing_char = match i {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        };
        missing_chars.push(missing_char)
    }
    Some(missing_chars)
}

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut the_scores: Vec<usize> = lines
            .flatten()
            .filter_map(|x| brackets(x.chars()))
            .map(score)
            .collect();
        the_scores.sort_unstable();
        let middle_value = the_scores[the_scores.len() / 2 | 0];
        println!("The answer is: {:?}", middle_value)
    }
}
