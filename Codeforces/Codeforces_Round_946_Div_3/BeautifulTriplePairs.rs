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

        let mut _ff = HashMap::<(i32, i32), i32>::new();
        let mut _ss = HashMap::<(i32, i32), i32>::new();
        let mut _tt = HashMap::<(i32, i32), i32>::new();

        let mut ff = HashMap::<(i32, i32), HashMap<i32, i32>>::new();
        let mut ss = HashMap::<(i32, i32), HashMap<i32, i32>>::new();
        let mut tt = HashMap::<(i32, i32), HashMap<i32, i32>>::new();

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<i32>();
            arr.push(x);
        }

        for y in arr.windows(3) {
            let a = y[0];
            let b = y[1];
            let c = y[2];

            *_ff.entry((b, c)).or_insert(0) += 1;

            *_ss.entry((a, c)).or_insert(0) += 1;

            *_tt.entry((a, b)).or_insert(0) += 1;

            *ff.entry((b, c))
                .or_insert(HashMap::new())
                .entry(a)
                .or_insert(0) += 1;

            *ss.entry((a, c))
                .or_insert(HashMap::new())
                .entry(b)
                .or_insert(0) += 1;

            *tt.entry((a, b))
                .or_insert(HashMap::new())
                .entry(c)
                .or_insert(0) += 1;
        }

        let mut ans = 0;
        for y in arr.windows(3) {
            let a = y[0];
            let b = y[1];
            let c = y[2];

            {
                let x = ff.get(&(b, c)).unwrap();
                let y = x.get(&a).unwrap();
                let z = _ff.get(&(b, c)).unwrap();

                let dx = (*z - *y) as i128;
                ans += dx;
            }

            {
                let x = ss.get(&(a, c)).unwrap();
                let y = x.get(&b).unwrap();
                let z = _ss.get(&(a, c)).unwrap();

                let dx = (*z - *y) as i128;
                ans += dx;
            }

            {
                let x = tt.get(&(a, b)).unwrap();
                let y = x.get(&c).unwrap();
                let z = _tt.get(&(a, b)).unwrap();

                let dx = (*z - *y) as i128;
                ans += dx;
            }
        }

        ans /= 2;

        println!("{ans}");
    }
}
