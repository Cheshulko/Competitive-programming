// https://leetcode.com/problems/maximum-number-of-points-with-cost

mod cm_lazy_seg_tree_capture {
    use std::ops::Range;

    // T - node value, TL - lazy value
    // [l, r)
    pub struct LazySegmentTree<F, M, L, T: Default + Copy, TL: Copy>
    where
        F: Fn(&T, &T) -> T,                 // left, right
        M: Fn(&TL, &TL) -> TL,              // old lazy, new lazy
        L: Fn(&T, &Range<usize>, &TL) -> T, // node value, node range, lazy value
    {
        len: usize,
        tree: Vec<T>,
        lazy: Vec<Option<TL>>,
        merge: F,
        merge_lazy: M,
        apply_lazy: L,
    }

    impl<F, M, L, T: Default + Copy, TL: Copy> LazySegmentTree<F, M, L, T, TL>
    where
        F: Fn(&T, &T) -> T,
        M: Fn(&TL, &TL) -> TL,
        L: Fn(&T, &Range<usize>, &TL) -> T,
    {
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
        pub fn query(&mut self, range: Range<usize>) -> Option<T> {
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
        ) -> Option<T> {
            if element_range.start >= query_range.end || element_range.end <= query_range.start {
                return None;
            }
            if element_range.start >= query_range.start && element_range.end <= query_range.end {
                return Some(self.tree[idx]);
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            self.propagate(idx, &element_range);
            let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range);
            let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
            match (left, right) {
                (None, None) => None,
                (None, Some(r)) => Some(r),
                (Some(l), None) => Some(l),
                (Some(l), Some(r)) => Some((self.merge)(&l, &r)),
            }
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

struct Solution {}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points.len();
        let m = points[0].len();

        let mut dp = vec![vec![i64::MIN; m]; n];
        for j in 0..m {
            dp[0][j] = points[0][j] as i64;
        }
        for i in 1..n {
            let arr = (0..m)
                .map(|x| dp[i - 1][x] - 1 * (x as i64))
                .collect::<Vec<_>>();
            let mut tree = cm_lazy_seg_tree_capture::LazySegmentTree::from_vec(
                &arr,
                0,
                |&l, &r| l.max(r),
                |&lazy1: &i64, &lazy2| lazy1 + lazy2,
                |&node_val, _, &lazy| node_val + lazy,
            );
            for j in 0..m {
                dp[i][j] = dp[i][j].max(points[i][j] as i64 + tree.query(0..m).unwrap());
                tree.update(0..(j + 1), -1);
                tree.update((j + 1)..m, 1);
            }
        }

        let mut ans = i64::MIN;
        for j in 0..m {
            ans = ans.max(dp[n - 1][j]);
        }

        ans
    }
}
