#[derive(Copy, Clone, Debug, Default)]
struct Cell {
    value: u8,
    drawn: bool,
}

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        let value = s.parse::<u8>().unwrap();
        Self {
            value,
            ..Default::default()
        }
    }
}
#[derive(Debug)]
struct Board {
    layout: Vec<Vec<Cell>>,
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let layout = s
            .split('\n')
            .map(|x| {
                let mut v: Vec<Cell> = Vec::new();
                for i in x.split(' ') {
                    if i.is_empty() {
                        continue;
                    }
                    v.push(Cell::from(i))
                }
                v
            })
            .collect();
        Self { layout }
    }
}

impl Board {
    fn mark(&mut self, value: u8) {
        for row in self.layout.iter_mut() {
            for mut cell in row {
                if cell.value == value {
                    cell.drawn = true;
                }
            }
        }
    }

    fn unmarked_sum(&self) -> usize {
        let mut unmarked: Vec<usize> = Vec::new();
        for row in &self.layout {
            for cell in row {
                if !cell.drawn {
                    unmarked.push(cell.value as usize);
                }
            }
        }
        unmarked.iter().sum()
    }

    fn is_winner(&self) -> bool {
        for row in &self.layout {
            if row.iter().all(|x| x.drawn) {
                return true;
            }
        }
        for i in 0..self.layout.len() {
            let mut n: Vec<Cell> = Vec::new();
            for v in 0..self.layout[i].len() {
                n.push(self.layout[v][i]);
            }
            if n.iter().all(|x| x.drawn) {
                return true;
            }
        }
        false
    }
}

fn parse_input(d: &str) -> (Vec<u8>, Vec<Board>) {
    let mut data = d.split("\n\n");
    let draws = data
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let boards = data.map(Board::from).collect();
    (draws, boards)
}

fn main() {
    let data: &str = include_str!("../input");
    let (draws, mut boards) = parse_input(data);
    let mut score: usize = 0;

    'draws: for draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
            if board.is_winner() {
                let sum = board.unmarked_sum();
                score = draw as usize * sum;
                break 'draws;
            }
        }
    }

    println!("The answer is: {}", score);
}
