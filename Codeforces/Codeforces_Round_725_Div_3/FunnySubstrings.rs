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

#[derive(Clone, Debug)]
struct Var {
    counts: usize,
    pref: String,
    suf: String,
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut vars = HashMap::<String, Var>::new();
        let mut ans = 0;
        for _ in 0..n {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();

            let mut is = s.split_whitespace();

            let name = is.next().unwrap().trim();
            let t = is.next().unwrap().trim();

            if t == ":=" {
                let value = is.next().unwrap().trim();

                let counts = value.contains(&"haha") as usize;
                ans = counts;

                vars.insert(
                    name.to_string(),
                    Var {
                        counts: counts,
                        pref: (&value[0..3.min(value.len())]).to_string(),
                        suf: (&value[value.len().checked_sub(3).unwrap_or(0)..]).to_string(),
                    },
                );
            } else {
                let first = is.next().unwrap().trim();
                let _ = is.next().unwrap();
                let second = is.next().unwrap().trim();

                let first = vars.get(first).unwrap().clone();
                let second = vars.get(second).unwrap().clone();

                let to_insert = {
                    let s = format!("{x}{y}", x = first.suf.clone(), y = second.pref.clone());

                    let mut counts = first.counts + second.counts;
                    if s.len() >= 4 {
                        for i in 0..=(s.len() - 4) {
                            counts += *(&s[i..(i + 4)].contains(&"haha")) as usize;
                        }
                    }

                    let pref = format!("{x}{y}", x = first.pref.clone(), y = second.pref.clone());
                    let suf = format!("{x}{y}", x = first.suf.clone(), y = second.suf.clone());

                    ans = counts;

                    Var {
                        counts: counts,
                        pref: (&pref[0..3.min(pref.len())]).to_string(),
                        suf: (&suf[suf.len().checked_sub(3).unwrap_or(0)..]).to_string(),
                    }
                };

                vars.insert(name.to_string(), to_insert);
            }
        }

        println!("{ans}");
    }
}
