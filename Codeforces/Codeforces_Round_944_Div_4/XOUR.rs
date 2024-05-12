use std::cmp::*;
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
        let n = cin.next::<usize>();

        let mut arr = vec![];
        for i in 0..n {
            let x = cin.next::<usize>();
            arr.push((x, i));
        }

        let mut hm = HashMap::<usize, Vec<(usize, usize)>>::new();
        for y @ (x, _) in arr.into_iter() {
            if x < 4 {
                hm.entry(0).or_default().push(y);
            } else {
                let h = x >> 2;
                assert!(h > 0);
                hm.entry(h).or_default().push(y);
            }
        }

        let mut ans = vec![0; n];
        let groups = hm.into_values().collect::<Vec<_>>();
        for mut group in groups.into_iter() {
            group.sort_unstable();

            let mut values = group.iter().map(|(v, _)| *v).collect::<Vec<_>>();
            let mut indexes = group.into_iter().map(|(_, i)| i).collect::<Vec<_>>();

            values.sort_unstable();
            indexes.sort_unstable();

            for (i, ind) in indexes.into_iter().enumerate() {
                ans[ind] = values[i];
            }
        }

        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
