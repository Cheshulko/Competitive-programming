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

mod cm {
    use std::cmp::min;
    use std::fmt::Debug;
    use std::ops::Range;

    pub struct SegmentTree<T: Debug + Default + Ord + Copy> {
        len: usize,
        tree: Vec<T>,
        merge: fn(T, T) -> T,
    }

    impl<T: Debug + Default + Ord + Copy> SegmentTree<T> {
        pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
            let len = arr.len();
            let mut buf: Vec<T> = vec![T::default(); 2 * len];
            buf[len..(2 * len)].clone_from_slice(&arr[0..len]);
            for i in (1..len).rev() {
                buf[i] = merge(buf[2 * i], buf[2 * i + 1]);
            }
            SegmentTree {
                len,
                tree: buf,
                merge,
            }
        }

        pub fn query(&self, range: Range<usize>) -> Option<T> {
            let mut l = range.start + self.len;
            let mut r = min(self.len, range.end) + self.len;
            let mut res = None;
            while l < r {
                if l % 2 == 1 {
                    res = Some(match res {
                        None => self.tree[l],
                        Some(old) => (self.merge)(old, self.tree[l]),
                    });
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    res = Some(match res {
                        None => self.tree[r],
                        Some(old) => (self.merge)(old, self.tree[r]),
                    });
                }
                l /= 2;
                r /= 2;
            }
            res
        }

        pub fn update(&mut self, idx: usize, val: T) {
            let mut idx = idx + self.len;
            self.tree[idx] = val;

            idx /= 2;
            while idx != 0 {
                self.tree[idx] = (self.merge)(self.tree[2 * idx], self.tree[2 * idx + 1]);
                idx /= 2;
            }
        }
    }
}

fn main() {
    let mut cin = Cin::new();
    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let s = cin.next::<String>().into_bytes();
        let n = s.len();

        let mut pref: Vec<i32> = vec![0; n + 1];
        for i in 0..n {
            pref[i + 1] = pref[i] + if s[i] == b'(' { 1 } else { -1 };
        }

        let max_seg_tree = cm::SegmentTree::from_vec(&pref, max);

        let mut pos = HashMap::<usize, Vec<usize>>::new();
        for i in 0..n {
            pos.entry(pref[i + 1] as usize).or_default().push(i + 1);
        }

        let mut ans = 0;
        for (p, cnt) in pos.into_iter() {
            let mut i = 0;

            let n = cnt.len();

            while i < n {
                let mut j = i;

                while j + 1 < n {
                    let g = max_seg_tree.query(cnt[i]..(cnt[j + 1] + 1)).unwrap() as usize;
                    if g <= 2 * p {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if j > i {
                    let d = j - i;
                    ans += d * (d + 1) / 2;
                }
                i = j + 1;
            }
        }

        println!("{ans}");
    }
}
