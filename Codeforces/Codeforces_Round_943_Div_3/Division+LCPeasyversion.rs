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

fn match_with_z_array<T: Eq>(
    input_string: &[T],
    pattern: &[T],
    start_index: usize,
    only_full_matches: bool,
) -> Vec<usize> {
    let size = input_string.len();
    let pattern_size = pattern.len();
    let mut last_match: usize = 0;
    let mut match_end: usize = 0;
    let mut array = vec![0usize; size];
    for i in start_index..size {
        if i <= match_end {
            array[i] = std::cmp::min(array[i - last_match], match_end - i + 1);
        }
        while (i + array[i]) < size && array[i] < pattern_size {
            if input_string[i + array[i]] != pattern[array[i]] {
                break;
            }
            array[i] += 1;
        }
        if (i + array[i]) > (match_end + 1) {
            match_end = i + array[i] - 1;
            last_match = i;
        }
    }
    if !only_full_matches {
        array
    } else {
        let mut answer: Vec<usize> = vec![];
        for (idx, number) in array.iter().enumerate() {
            if *number == pattern_size {
                answer.push(idx);
            }
        }
        answer
    }
}

#[allow(dead_code)]
pub fn z_array<T: Eq>(input: &[T]) -> Vec<usize> {
    match_with_z_array(input, input, 1, false)
}

pub fn match_pattern<T: Eq>(input: &[T], pattern: &[T]) -> Vec<usize> {
    match_with_z_array(input, pattern, 0, true)
}

fn main() {
    let mut cin = Cin::new();
    let t = cin.next::<usize>();

    for _ in 0..t {
        let _n = cin.next::<usize>();
        let (l, _r) = (cin.next::<usize>(), cin.next::<usize>());
        let s = cin.next::<String>();

        let k = l;

        if k == 1 {
            println!("{x}", x = s.len());
            continue;
        }

        let arr = z_array(s.as_bytes());
        let n = arr.len();

        let mut r = arr.iter().max().unwrap() + 1;
        let mut l = 0;

        while r - l > 1 {
            let mut prev = 0;
            let mut cnt = 0;
            let m = (r + l) / 2;
            for i in 0..n {
                if arr[i] >= m && i - prev >= m {
                    cnt += 1;
                    prev = i;
                }
            }

            if cnt + 1 >= k {
                l = m;
            } else {
                r = m;
            }
        }

        println!("{l}");
    }
}
