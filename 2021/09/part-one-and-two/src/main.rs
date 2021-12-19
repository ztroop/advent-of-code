struct HeightMap {
    grid: Vec<Vec<i64>>,
    width: usize,
    height: usize,
}

impl From<&str> for HeightMap {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<i64>> = s
            .trim()
            .split('\n')
            .map(|x| {
                let mut v: Vec<i64> = Vec::new();
                for i in x.split_terminator("").skip(1) {
                    v.push(i.parse().unwrap())
                }
                v
            })
            .collect();
        let width: usize = grid[0].len();
        let height: usize = grid.len();

        HeightMap {
            grid,
            width,
            height,
        }
    }
}

impl HeightMap {
    fn neighbours(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let mut neighbours: Vec<(isize, isize)> = Vec::new();

        for [a, b] in [[1, 0], [-1, 0], [0, -1], [0, 1]] {
            if !(a == 0 && b == 0) {
                let nx = x + a;
                let ny = y + b;

                if (nx >= 0 && nx < self.width as isize) && (ny >= 0 && ny < self.height as isize) {
                    neighbours.push((nx, ny))
                }
            }
        }

        neighbours
    }

    fn smallest(&self, value: usize, neighbours: Vec<(isize, isize)>) -> bool {
        let neighbours: Vec<usize> = neighbours
            .iter()
            .map(|(col, row)| self.grid[*row as usize][*col as usize].try_into().unwrap())
            .collect();

        if value < *neighbours.iter().min().unwrap() {
            return true;
        }
        false
    }
}

fn main() {
    let data = include_str!("../input");
    let layout = HeightMap::from(data);
    let mut numbers: Vec<i64> = Vec::new();

    for (row, y) in layout.grid.iter().enumerate() {
        for (col, _) in y.iter().enumerate() {
            let current_value: i64 = layout.grid[row][col];
            let neighbours = layout.neighbours(col as isize, row as isize);
            if layout.smallest(current_value as usize, neighbours) {
                numbers.push(current_value)
            }
        }
    }

    println!(
        "The answer is: {:?}",
        numbers.iter().sum::<i64>() + numbers.iter().count() as i64
    )
}
