use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::usize;
use std::vec;

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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

mod cm_seg_tree {
    use std::ops::Range;

    // [l, r)
    pub struct SegmentTree<T: Default + Copy> {
        len: usize,
        tree: Vec<T>,
        merge: fn(T, T) -> T,
    }

    impl<T: Default + Copy> SegmentTree<T> {
        pub fn from_vec(arr: &[T], default: T, merge: fn(T, T) -> T) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = SegmentTree {
                len,
                tree: vec![default; 2 * pow_2_len - 1],
                merge,
            };

            sgtr.build_recursive(arr, 0, 0..len);
            sgtr
        }

        // range: [l, r). O(log(n))
        pub fn query(&self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
        }

        // range: [l, r). O(log(n))
        pub fn update(&mut self, idx: usize, val: T) {
            self.update_recursive(0, 0..self.len, idx, val);
        }

        fn build_recursive(&mut self, arr: &[T], idx: usize, range: Range<usize>) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end);
                self.tree[idx] = (self.merge)(self.tree[2 * idx + 1], self.tree[2 * idx + 2]);
            }
        }

        fn query_recursive(
            &self,
            idx: usize,
            element_range: Range<usize>,
            query_range: &Range<usize>,
        ) -> Option<T> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }
            if element_range.start >= query_range.start && element_range.end <= query_range.end {
                return Some(self.tree[idx]);
            }
            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            match (left, right) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => Some((self.merge)(l, r)),
            }
        }

        fn update_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            element_idx: usize,
            val: T,
        ) {
            if element_range.start > element_idx || element_range.end <= element_idx {
                return;
            }
            if element_range.end - element_range.start == 1 && element_range.start == element_idx {
                self.tree[idx] = val;
                return;
            }
            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.update_recursive(idx * 2 + 1, element_range.start..mid, element_idx, val);
            self.update_recursive(idx * 2 + 2, mid..element_range.end, element_idx, val);
            self.tree[idx] = (self.merge)(self.tree[idx * 2 + 1], self.tree[idx * 2 + 2]);
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![0; n];
        let mut ind = vec![VecDeque::new(); n + 1];
        let mut ind_last = vec![0; n + 1];

        for i in 0..n {
            a[i] = cin.next::<usize>();
            ind[a[i]].push_back(i);
            ind_last[a[i]] = i + 1;
        }

        let mut ind_last_sort = BTreeSet::<usize>::new();
        for &ind in ind_last.iter() {
            if ind != 0 {
                ind_last_sort.insert(ind - 1);
            }
        }

        let mut used = vec![false; n];
        let mut tree_min =
            cm_seg_tree::SegmentTree::from_vec(&a, usize::MAX, |left, right| left.min(right));
        let mut tree_max = cm_seg_tree::SegmentTree::from_vec(&a, 0, |left, right| left.max(right));

        let mut ans = vec![];
        let mut rev = true;
        let mut cur_ind = 0;

        while !ind_last_sort.is_empty() {
            let last = ind_last_sort.first().unwrap();

            let v = if rev {
                tree_max.query(cur_ind..(last + 1)).unwrap()
            } else {
                tree_min.query(cur_ind..(last + 1)).unwrap()
            };

            ans.push(v);
            for &ii in ind[v].iter() {
                tree_min.update(ii, usize::MAX);
                tree_max.update(ii, 0);
                used[ii] = true;
            }
            ind_last_sort.remove(&(ind_last[v] - 1));

            while cur_ind < n && a[cur_ind] != v {
                cur_ind += 1;
            }

            while cur_ind < n && used[cur_ind] {
                cur_ind += 1;
            }

            rev = !rev;
        }

        println!("{l}", l = ans.len());
        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
