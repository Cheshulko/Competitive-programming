// https://leetcode.com/problems/fruits-into-baskets-iii

mod cm_seg_tree {
    use std::fmt::Debug;
    use std::ops::Range;

    // [l, r)
    pub struct SegmentTree<T: Debug + Default + Copy> {
        len: usize,
        tree: Vec<T>,
        merge: fn(T, T) -> T,
    }

    impl<T: Debug + Default + Copy> SegmentTree<T> {
        pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
            let len = arr.len();

            let mut pow_2 = len.ilog2() as usize;
            if len & (len - 1) != 0 {
                pow_2 += 1;
            }
            let pow_2_len = 1 << pow_2;

            let mut sgtr = SegmentTree {
                len,
                tree: vec![T::default(); 2 * pow_2_len - 1],
                merge,
            };

            sgtr.build_recursive(arr, 0, 0..len, merge);
            sgtr
        }

        // range: [l, r)
        pub fn query(&self, range: Range<usize>) -> Option<T> {
            self.query_recursive(0, 0..self.len, &range)
        }

        pub fn update(&mut self, idx: usize, val: T) {
            self.update_recursive(0, 0..self.len, idx, val);
        }

        fn build_recursive(
            &mut self,
            arr: &[T],
            idx: usize,
            range: Range<usize>,

            merge: fn(T, T) -> T,
        ) {
            if range.end - range.start == 1 {
                self.tree[idx] = arr[range.start];
            } else {
                let mid = range.start + (range.end - range.start) / 2;
                self.build_recursive(arr, 2 * idx + 1, range.start..mid, merge);
                self.build_recursive(arr, 2 * idx + 2, mid..range.end, merge);
                self.tree[idx] = merge(self.tree[2 * idx + 1], self.tree[2 * idx + 2]);
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

    impl<T: Debug + Default + Ord + Copy> SegmentTree<T> {
        pub fn lower_bound(&self, x: T) -> Option<usize> {
            self.lower_bound_recursive(x, 0, 0..self.len)
        }

        fn lower_bound_recursive(
            &self,
            x: T,
            idx: usize,
            element_range: Range<usize>,
        ) -> Option<usize> {
            if element_range.end - element_range.start == 1 {
                if self.tree[idx] >= x {
                    return Some(element_range.start);
                } else {
                    return None;
                }
            }

            let mid = element_range.start + (element_range.end - element_range.start) / 2;
            let left = self.tree[2 * idx + 1];
            let right = self.tree[2 * idx + 2];
            if left >= x {
                self.lower_bound_recursive(x, 2 * idx + 1, element_range.start..mid)
            } else if right >= x {
                self.lower_bound_recursive(x, 2 * idx + 2, mid..element_range.end)
            } else {
                None
            }
        }
    }
}

struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut tree = cm_seg_tree::SegmentTree::from_vec(&baskets, |left, right| left.max(right));

        let n = fruits.len();
        let mut can = 0;

        for fruit in fruits.into_iter() {
            if let Some(ind) = tree.lower_bound(fruit) {
                can += 1;
                tree.update(ind, 0);
            }
        }

        (n - can) as i32
    }
}
