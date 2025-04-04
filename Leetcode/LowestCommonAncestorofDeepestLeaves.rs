// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            cur: Option<Rc<RefCell<TreeNode>>>,
            depth: usize,
        ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
            if let Some(cur_) = cur.as_ref() {
                let cur_ = cur_.borrow();

                let (left, depth_left) = dfs(cur_.left.clone(), depth + 1);
                let (right, depth_right) = dfs(cur_.right.clone(), depth + 1);

                match (left.as_ref(), right.as_ref()) {
                    (None, None) => (cur.clone(), depth),
                    (None, Some(_)) => (right, depth_right),
                    (Some(_), None) => (left, depth_left),
                    (Some(_), Some(_)) => match depth_left.cmp(&depth_right) {
                        Ordering::Equal => (cur.clone(), depth_left),
                        Ordering::Less => (right, depth_right),
                        Ordering::Greater => (left, depth_left),
                    },
                }
            } else {
                return (None, usize::MIN);
            }
        }

        dfs(root, 0).0
    }
}
