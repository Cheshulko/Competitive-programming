use std::cmp::*;
use std::collections::*;
use std::io::stdin;
use std::slice::*;

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
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();

        let mut d: Vec<i32> = vec![0; 4];

        for &c in &s {
            match c {
                b'N' => d[0] += 1,
                b'S' => d[1] += 1,
                b'E' => d[2] += 1,
                b'W' => d[3] += 1,
                _ => unreachable!(),
            }
        }

        let dy = d[0] - d[1];
        let dx = d[2] - d[3];

        let _dx: i32 = dx.abs();
        let _dy: i32 = dy.abs();

        if _dx % 2 == 1 || _dy % 2 == 1 || (d[0] + d[1] + d[2] + d[3] == 2 && _dx == 0 && _dy == 0)
        {
            println!("NO");
        } else {
            let y1 = d[0].min(d[1]);
            let y2 = (d[0].max(d[1]) - y1) / 2;

            let mut ry = [y1 + y2, y1 + y2];
            let mut hy = [y2, y2];

            let mut sw = false;
            if hy[0] == 0 && ry[1] > 0 {
                hy[0] += 1;
                ry[1] -= 1;
                sw = true;
            }
            if hy[1] == 0 && ry[0] > 0 {
                hy[1] += 1;
                ry[0] -= 1;
                sw = true;
            }

            let x1 = d[2].min(d[3]);
            let x2 = (d[2].max(d[3]) - x1) / 2;

            let mut rx = [x1 + x2, x1 + x2];
            let mut hx = [x2, x2];

            if hx[0] == 0 && !sw {
                hx[0] += 1;
                rx[1] -= 1;
            }
            if hx[1] == 0 && !sw {
                hx[1] += 1;
                rx[0] -= 1;
            }

            let mut ans = vec![];

            for c in s.into_iter() {
                match c {
                    b'N' => {
                        if ry[0] > 0 {
                            ry[0] -= 1;
                            ans.push('R');
                        } else {
                            hy[0] -= 1;
                            ans.push('H');
                        }
                    }
                    b'S' => {
                        if ry[1] > 0 {
                            ry[1] -= 1;
                            ans.push('R');
                        } else {
                            hy[1] -= 1;
                            ans.push('H');
                        }
                    }
                    b'E' => {
                        if rx[0] > 0 {
                            rx[0] -= 1;
                            ans.push('R');
                        } else {
                            hx[0] -= 1;
                            ans.push('H');
                        }
                    }
                    b'W' => {
                        if rx[1] > 0 {
                            rx[1] -= 1;
                            ans.push('R');
                        } else {
                            hx[1] -= 1;
                            ans.push('H');
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let ans = ans.into_iter().collect::<String>();
            println!("{ans}");
        }
    }
}
