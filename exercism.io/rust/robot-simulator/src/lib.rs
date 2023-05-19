// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const DIRS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    fn dx(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 0,
            Direction::West => -1,
        }
    }

    fn dy(&self) -> i32 {
        match self {
            Direction::North => 1,
            Direction::East => 0,
            Direction::South => -1,
            Direction::West => 0,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    cur_direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x,
            y,
            cur_direction: d,
        }
    }

    fn cur_direction_index(&self) -> usize {
        Direction::DIRS
            .iter()
            .enumerate()
            .find(|(_, d)| d == &&self.cur_direction)
            .unwrap()
            .0
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        let len = Direction::DIRS.len();
        let ind = (self.cur_direction_index() + 1) % len;
        self.cur_direction = Direction::DIRS[ind];
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        let len = Direction::DIRS.len();
        let ind = (self.cur_direction_index() + len - 1) % len;
        self.cur_direction = Direction::DIRS[ind];
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        self.x += self.cur_direction.dx();
        self.y += self.cur_direction.dy();
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |mut robot, c| {
            robot = match c {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!(),
            };
            robot
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.cur_direction
    }
}
