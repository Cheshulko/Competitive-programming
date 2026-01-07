// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree

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
    const M: usize = 1_000_000_000 + 7;

    fn sum(root: Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(root) = root {
            let root = root.borrow();

            root.val as usize + Solution::sum(root.left.clone()) + Solution::sum(root.right.clone())
        } else {
            0
        }
    }

    fn solve(root: Option<Rc<RefCell<TreeNode>>>, all: usize, best: &mut usize) -> usize {
        if let Some(root) = root {
            let root = root.borrow();

            let tree_sum = (root.val as usize
                + Solution::solve(root.left.clone(), all, best)
                + Solution::solve(root.right.clone(), all, best));

            let rest = all - tree_sum;

            *best = (*best).max(tree_sum * rest);

            tree_sum
        } else {
            0
        }
    }

    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let all = Solution::sum(root.clone());

        let mut best = 0;
        Solution::solve(root, all, &mut best);

        (best % Solution::M) as i32
    }
}
