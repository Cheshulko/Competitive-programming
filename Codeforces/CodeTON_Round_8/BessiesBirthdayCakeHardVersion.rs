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
        let (n, x, mut y) = (cin.next::<usize>(), cin.next::<usize>(), cin.next::<i32>());

        let mut ans = 0;

        let mut points = vec![];
        for _ in 0..x {
            let p = cin.next::<usize>();
            points.push(p - 1);
        }

        points.sort_unstable();

        let mut can_insert = vec![];
        let p_len = points.len();
        for i in 0..p_len {
            if y == 0 {
                break;
            }

            let cur = points[i];
            let last = points[(i + 1) % p_len];

            let cnt = if last < cur {
                n - 1 - cur + last
            } else {
                last - cur - 1
            } as i32;

            if cnt <= 0 {
                continue;
            }

            can_insert.push(cnt);
        }

        can_insert.sort_by(|a, b| match (a % 2 == 1, b % 2 == 1) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (_, _) => a.cmp(b),
        });

        for cnt in can_insert.into_iter() {
            let cnt_to_close = cnt / 2;

            if y > 0 && cnt_to_close > 0 {
                if cnt_to_close <= y {
                    ans += (cnt % 2 == 1) as i32 + cnt_to_close * 2;
                    y -= cnt_to_close;
                } else {
                    ans += y * 2;
                    y = 0;
                }
            }
        }

        fn next(pos: usize, points: &Vec<usize>) -> usize {
            let l = points.len();
            let mut to = (pos + 1) % l;

            while points[to] == usize::MAX {
                to += 1;
                to %= l;
            }

            to
        }

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
