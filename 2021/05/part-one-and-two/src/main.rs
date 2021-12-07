#![allow(dead_code)]

use itertools::Itertools;
use std::{
    iter::{repeat, Chain, Rev},
    ops::RangeInclusive,
};

type Route = Vec<(usize, usize)>;
type Board = Vec<Vec<usize>>;

enum Mode {
    Normal,
    Diagonal,
}

#[derive(Debug)]
struct Coordinate {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl From<&'static str> for Coordinate {
    fn from(s: &'static str) -> Self {
        let (x1, y1, x2, y2) = s
            .split(" -> ")
            .flat_map(|point| point.split(','))
            .map(|coord| coord.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x1, y1, x2, y2 }
    }
}

struct Grid {
    array: Board,
    mode: Mode,
}

impl Grid {
    fn new(x: usize, y: usize, mode: Mode) -> Self {
        let mut array = Vec::with_capacity(y);
        for _ in 0..y {
            array.push([0].repeat(x));
        }
        Self { array, mode }
    }

    fn populate(&mut self, c: Coordinate) {
        let mut plot = |r: Route| r.into_iter().map(|x| self.array[x.0][x.1] += 1).collect();

        let route = |start: usize,
                     stop: usize|
         -> Chain<RangeInclusive<usize>, Rev<RangeInclusive<usize>>> {
            (start..=stop).chain((stop..=start).rev())
        };

        if c.x1 == c.x2 {
            plot(repeat(c.x1).zip(route(c.y1, c.y2)).collect())
        } else if c.y1 == c.y2 {
            plot(route(c.x1, c.x2).zip(repeat(c.y1)).collect())
        } else if let Mode::Diagonal = self.mode {
            plot(route(c.x1, c.x2).zip(route(c.y1, c.y2)).collect())
        }
    }

    fn count(&self) -> usize {
        let danger_level: usize = 2;
        self.array
            .clone()
            .into_iter()
            .flatten()
            .filter(|&v| v >= danger_level)
            .count()
    }
}

fn main() {
    let data = include_str!("../input").lines();
    let mut grid = Grid::new(999, 999, Mode::Normal);

    for l in data {
        grid.populate(Coordinate::from(l));
    }

    println!("The answer is: {}", grid.count())
}
