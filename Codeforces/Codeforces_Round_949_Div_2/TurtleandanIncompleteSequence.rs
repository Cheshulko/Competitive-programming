use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut b = vec![];
        let mut a = vec![-1; n];
        let mut indx = vec![];
        for i in 0..n {
            let x = cin.next::<i64>();
            b.push(x);

            if x != -1 {
                indx.push(i);
                a[i] = x;
            }
        }

        if indx.len() == 0 {
            let mut cur = 1;
            let mut l = true;

            for i in 0..n {
                a[i] = cur;
                if l {
                    cur *= 2;
                } else {
                    cur /= 2;
                }
                l = !l;

                print!("{x} ", x = a[i]);
            }
            println!();

            continue;
        }

        if indx.len() > 0 {
            let first = indx[0];
            if first > 0 {
                let mut cur = b[first];
                let mut l = if cur != 1 { true } else { false };
                for i in (0..first).rev() {
                    if l {
                        cur /= 2;
                    } else {
                        cur *= 2;
                    }
                    a[i] = cur;
                    l = !l;
                }
            }

            let last = indx[indx.len() - 1];
            if last < n - 1 {
                let mut cur = b[last];
                let mut l = if cur != 1 { true } else { false };
                for i in (last + 1)..n {
                    if l {
                        cur /= 2;
                    } else {
                        cur *= 2;
                    }
                    a[i] = cur;

                    l = !l;
                }
            }
        }

        let mut can = true;
        for x in indx.windows(2) {
            let mut next_ind = x[1];
            let mut prev_ind = x[0];

            let mut next = b[next_ind];
            let mut prev = b[prev_ind];

            if next_ind - prev_ind == 1 {
                if !((next / 2 == prev) || (prev / 2 == next)) {
                    can = false;
                    break;
                }
            } else {
                while prev != next && prev_ind != next_ind {
                    if next > prev {
                        next_ind -= 1;
                        next /= 2;
                        a[next_ind] = next;
                    } else {
                        prev_ind += 1;
                        prev /= 2;
                        a[prev_ind] = prev;
                    }
                }

                if prev == next {
                    if (next_ind - prev_ind) % 2 == 1 {
                        can = false;
                        break;
                    } else {
                        let mut l = true;
                        while prev_ind != next_ind {
                            prev_ind += 1;
                            if l {
                                prev *= 2;
                            } else {
                                prev /= 2;
                            }
                            a[prev_ind] = prev;
                            l = !l;
                        }
                    }
                } else {
                    can = false;
                    break;
                }
            }
        }

        if !can {
            println!("-1");
        } else {
            for i in 0..n {
                print!("{x} ", x = a[i]);
            }
            println!();
        }
    }
}
