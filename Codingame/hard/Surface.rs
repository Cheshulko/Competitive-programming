// https://www.codingame.com/ide/puzzle/surface

use std::fmt;
use std::io;
use std::ops;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Add<&Point> for &Point {
    type Output = Point;

    fn add(self, _rhs: &Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

struct Grid {
    field: Vec<String>,
    value: Vec<Vec<u32>>,
    a_x: Vec<Vec<usize>>,
    a_y: Vec<Vec<usize>>,

    l: usize,
    h: usize,
}

impl Grid {
    const DIRS: [Point; 4] = [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
    ];
}

impl Grid {
    fn at(&self, point: &Point) -> char {
        self.field[point.y as usize]
            .chars()
            .nth(point.x as usize)
            .unwrap()
    }

    fn is_valid(&self, point: &Point) -> bool {
        self.at(point) == 'O'
    }

    fn can_go(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.l as i32 && point.y >= 0 && point.y < self.h as i32
    }

    fn from_input() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let l = parse_input!(input_line, usize);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let h = parse_input!(input_line, usize);

        let mut grid = Grid {
            field: vec![],
            value: vec![vec![0; h]; l],
            a_x: vec![vec![0; h]; l],
            a_y: vec![vec![0; h]; l],
            l,
            h,
        };

        (0..h).for_each(|_| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim_matches('\n').to_string();
            grid.field.push(row);
        });

        grid
    }

    fn bfs(&mut self, active: &Point, coor: &Point, sum: &mut u32) {
        let mut v: Vec<Point> = vec![];
        v.push(*coor);

        self.a_x[coor.x as usize][coor.y as usize] = active.x as usize + 1;
        self.a_y[coor.x as usize][coor.y as usize] = active.y as usize + 1;

        while !v.is_empty() {
            let cur = v.pop().unwrap();
            *sum += 1;

            for dir in &Grid::DIRS {
                let to: Point = &cur + dir;

                if self.can_go(&to) && self.is_valid(&to) {
                    if self.a_x[to.x as usize][to.y as usize] == 0 {
                        self.a_x[to.x as usize][to.y as usize] = active.x as usize + 1;
                        self.a_y[to.x as usize][to.y as usize] = active.y as usize + 1;

                        v.push(to);
                    }
                }
            }
        }
    }

    fn ans(&mut self, point: Point) -> u32 {
        if self.is_valid(&point) {
            let px = point.x as usize;
            let py = point.y as usize;
            if self.a_x[px][py] > 0 {
                let x = (self.a_x[px][py] - 1) as usize;
                let y = (self.a_y[px][py] - 1) as usize;
                return self.value[x][y];
            }

            let mut sum: u32 = 0;
            self.bfs(&point, &point, &mut sum);
            self.value[point.x as usize][point.y as usize] = sum;
            sum
        } else {
            0
        }
    }

    fn solve(&mut self) {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let n = parse_input!(input_line, i32);

        let mut result: Vec<u32> = vec![];

        (0..n).for_each(|_| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);

            result.push(self.ans(Point { x, y }))
        });

        for it in result.iter() {
            println!("{}", it);
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid_str = String::new();
        grid_str.push_str("Grid\n");
        for it in self.field.iter() {
            grid_str.push_str(format!("{}\n", it).as_str());
        }
        write!(f, "{}", grid_str)
    }
}

fn main() {
    let mut grid = Grid::from_input();
    grid.solve();
}
