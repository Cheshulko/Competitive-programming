// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor

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
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::go(root).0
    }

    fn go(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<i32>, Option<i32>) {
        if let Some(root) = root {
            let root = root.borrow();

            let mut res = (0, Some(root.val), Some(root.val));
            let left = Solution::go(root.left.clone());
            let right = Solution::go(root.right.clone());

            let values = vec![left.1, left.2, right.1, right.2]
                .into_iter()
                .filter_map(|x| x)
                .collect::<Vec<_>>();

            if let Some(mn) = values.iter().min() {
                res.1 = Some(res.1.unwrap().min(*mn));
            }

            if let Some(mx) = values.iter().max() {
                res.2 = Some(res.2.unwrap().max(*mx));
            }

            res.0 = res.0.max(left.0);
            res.0 = res.0.max(right.0);
            res.0 = res.0.max((res.1.unwrap() - root.val).abs());
            res.0 = res.0.max((res.2.unwrap() - root.val).abs());

            return res;
        }

        (0, None, None)
    }
}
