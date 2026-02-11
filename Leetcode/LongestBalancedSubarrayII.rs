// https://leetcode.com/problems/longest-balanced-subarray-ii

mod cm_lazy_seg_tree_capture {
    use std::ops::Range;

    pub trait Queryable {
        fn query(&self) -> bool;
    }

    // T - node value, TL - lazy value
    // [l, r)
    pub struct LazySegmentTree<F, M, L, T, TL: Copy>
    where
        F: Fn(&T, &T) -> T,                 // left, right
        M: Fn(&TL, &TL) -> TL,              // old lazy, new lazy
        L: Fn(&T, &Range<usize>, &TL) -> T, // node value, node range, lazy value
        T: Copy + Queryable,
    {
        len: usize,
        tree: Vec<T>,
        lazy: Vec<Option<TL>>,
        merge: F,
        merge_lazy: M,
        apply_lazy: L,
    }

    impl<F, M, L, T, TL: Copy> LazySegmentTree<F, M, L, T, TL>
    where
        F: Fn(&T, &T) -> T,
        M: Fn(&TL, &TL) -> TL,
        L: Fn(&T, &Range<usize>, &TL) -> T,
        T: Copy + Queryable,
    {
        // O(n)
        pub fn from_vec(arr: &[T], default: T, merge: F, merge_lazy: M, apply_lazy: L) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = LazySegmentTree {
                len,
                tree: vec![default; 2 * pow_2_len - 1],
                lazy: vec![None; 2 * pow_2_len - 1],
                merge,
                merge_lazy,
                apply_lazy,
            };

            sgtr.build_recursive(arr, 0, 0..len);
            sgtr
        }

        // range: [l, r). O(log(n))
        pub fn query(&mut self, range: Range<usize>) -> Option<usize> {
            self.query_recursive(0, 0..self.len, &range)
        }

        // range: [l, r). O(log(n))
        pub fn update(&mut self, range: Range<usize>, lazy_val: TL) {
            self.update_recursive(0, 0..self.len, &range, lazy_val);
        }

        fn build_recursive(&mut self, arr: &[T], idx: usize, range: Range<usize>) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end);
                self.tree[idx] = (self.merge)(&self.tree[2 * idx + 1], &self.tree[2 * idx + 2]);
            }
        }

        fn query_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            query_range: &Range<usize>,
        ) -> Option<usize> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }

            if element_range.start + 1 == element_range.end
            /* Leaf */
            {
                if self.tree[idx].query() {
                    return Some(element_range.start);
                } else {
                    return None;
                }
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            if !self.tree[idx].query() {
                return None;
            }

            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            if right.is_some() {
                return right;
            }

            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            if left.is_some() {
                return left;
            }

            return None;
        }

        fn update_recursive(
            &mut self,
            idx: usize,
            element_range: Range<usize>,
            update_range: &Range<usize>,
            lazy_val: TL,
        ) {
            if element_range.start >= update_range.end || element_range.end <= update_range.start {
                return;
            }
            if element_range.end - element_range.start == 1 {
                self.apply(idx, &element_range, lazy_val);
                return;
            }
            if element_range.start >= update_range.start && element_range.end <= update_range.end {
                self.apply(idx, &element_range, lazy_val);
                return;
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            self.update_recursive(
                idx * 2 + 1,
                element_range.start..mid,
                update_range,
                lazy_val,
            );
            self.update_recursive(idx * 2 + 2, mid..element_range.end, update_range, lazy_val);
            self.tree[idx] = (self.merge)(&self.tree[idx * 2 + 1], &self.tree[idx * 2 + 2]);
        }

        fn apply(&mut self, idx: usize, element_range: &Range<usize>, lazy_val: TL) {
            self.tree[idx] = (self.apply_lazy)(&self.tree[idx], element_range, &lazy_val);
            self.lazy[idx] = match self.lazy[idx] {
                Some(lazy) => Some((self.merge_lazy)(&lazy, &lazy_val)),
                None => Some(lazy_val),
            };
        }

        fn propagate(&mut self, idx: usize, element_range: &Range<usize>) {
            if let Some(lazy) = self.lazy[idx] {
                let mid = element_range.start + (element_range.end - element_range.start) / 2;
                self.apply(2 * idx + 1, &(element_range.start..mid), lazy);
                self.apply(2 * idx + 2, &(mid..element_range.end), lazy);
            }
            self.lazy[idx] = None;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    mi: i32,
    ma: i32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            mi: i32::MAX / 2,
            ma: i32::MIN / 2,
        }
    }
}

impl cm_lazy_seg_tree_capture::Queryable for Node {
    fn query(&self) -> bool {
        self.mi <= 0 && 0 <= self.ma
    }
}

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        use std::collections::{HashMap, VecDeque};

        let n = nums.len();

        let mut pref: Vec<i32> = vec![0; n + 1];
        let mut seen = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            pref[i + 1] += pref[i];
            if !seen.contains_key(&num) {
                pref[i + 1] += [-1, 1][(num & 1) as usize];
            }
            seen.entry(num).or_insert_with(VecDeque::new).push_back(i);
        }

        let pref = pref
            .into_iter()
            .map(|p| Node { mi: p, ma: p })
            .collect::<Vec<_>>();

        let mut tree = cm_lazy_seg_tree_capture::LazySegmentTree::from_vec(
            &pref,
            Node::default(),
            |&left, &right| Node {
                mi: left.mi.min(right.mi),
                ma: left.ma.max(right.ma),
            },
            |&lazy_1: &i32, &lazy_2: &i32| lazy_1 + lazy_2,
            |&node_val, _, &lazy| Node {
                mi: node_val.mi + lazy,
                ma: node_val.ma + lazy,
            },
        );

        let mut ans = 0;
        for i in 0..n {
            if let Some(j) = tree.query(0 + ans..n + 1) {
                if j > i {
                    ans = ans.max(j - i);
                }
            }

            let num = nums[i];
            let ps = seen.get_mut(&num).unwrap();
            ps.pop_front();

            let end = *ps.front().unwrap_or(&n);
            tree.update(i + 1..(end + 1), [1, -1][(num & 1) as usize]);
        }

        ans as i32
    }
}
