use core::num;
use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &Path) -> Self {
        let tokens = VecDeque::new();
        let file = File::open(&path).expect("Expect file exists");
        Self {
            reader: Box::new(BufReader::new(file)),
            tokens,
        }
    }
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self {
            reader: Box::new(BufReader::new(std::io::stdin())),
            tokens,
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            self.reader.read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut hs = BTreeSet::<usize>::new();
        let mut mp = HashMap::<usize, usize>::new();

        hs.insert(0);
        let mut a = vec![(0, 0); n];
        for i in 0..n {
            let x = cin.next::<usize>();
            let y = cin.next::<usize>();
            a[i] = (x, y);

            hs.insert(x);
            hs.insert(y);
        }

        for x in hs.into_iter() {
            if !mp.contains_key(&x) {
                let len = mp.len();
                mp.insert(x, len);
            }
        }

        let mut max = 0;
        let mut arr = vec![0; 2 * 100_000 + 5];
        let mut aa = vec![(0, 0, 0); n];
        for i in 0..n {
            aa[i] = (*mp.get(&a[i].0).unwrap(), *mp.get(&a[i].1).unwrap(), i);

            arr[aa[i].0] += 1;
            arr[aa[i].1] += 1;

            max = max.max(aa[i].0).max(aa[i].1);
        }

        let mut pref = vec![0; max + 2];
        for i in 0..(max + 1) {
            pref[i + 1] = pref[i] + arr[i];
        }

        let mut sum = vec![0; n];
        for i in 0..n {
            sum[i] = pref[aa[i].0] + pref[aa[i].1];
        }

        aa.sort_by(|(x0, x1, i), (y0, y, j)| sum[*i].cmp(&sum[*j]));

        for i in 0..n {
            let ii = aa[i].2;
            print!("{} {} ", a[ii].0, a[ii].1);
        }
        println!();
    }

    Ok(())
}
