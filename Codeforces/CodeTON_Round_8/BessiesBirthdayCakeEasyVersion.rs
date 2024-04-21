use std::collections::*;
use std::io::stdin;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let t = cin.next::<usize>();

    for _ in 0..t {
        let (n, x, _) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut points = vec![];
        for _ in 0..x {
            let p = cin.next::<usize>();
            points.push(p - 1);
        }

        points.sort_unstable();

        fn next(pos: usize, points: &Vec<usize>) -> usize {
            let l = points.len();
            let mut to = (pos + 1) % l;

            while points[to] == usize::MAX {
                to += 1;
                to %= l;
            }

            to
        }

        let mut ans = 0;

        let pos_i = 0;
        let mut pos_j = 0;
        let mut pos_k = 0;

        loop {
            pos_j = next(pos_j, &points);
            pos_k = next(pos_j, &points);

            let i = points[pos_i];
            let j = points[pos_j];
            let k = points[pos_k];

            let tr = (i != j && j != k && i != k) as i32
                + ((i + 2) % n == j && points[(pos_i + 1) % points.len()] != usize::MAX) as i32
                + ((j + 2) % n == k && points[(pos_j + 1) % points.len()] != usize::MAX) as i32;

            ans += tr;

            points[pos_j] = usize::MAX;

            if pos_i == pos_k {
                break;
            }
        }

        println!("{ans}");
    }
}
