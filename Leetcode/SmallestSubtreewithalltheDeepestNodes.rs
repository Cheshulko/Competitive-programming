// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes

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

impl Solution {
    fn solve(
        root: Option<Rc<RefCell<TreeNode>>>,
        dep: i32,
        ans: &mut Option<Rc<RefCell<TreeNode>>>,
        ans_dep: &mut i32,
    ) -> i32 {
        if let Some(root_) = root.as_ref() {
            let root_borrow = root_.borrow();

            let left = Solution::solve(root_borrow.left.clone(), dep + 1, ans, ans_dep);
            let right = Solution::solve(root_borrow.right.clone(), dep + 1, ans, ans_dep);

            if left == right && right >= *ans_dep {
                *ans = root.clone();
                *ans_dep = left;
            }

            left.max(right)
        } else {
            dep
        }
    }

    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        let mut ans_dep = -1;
        Solution::solve(root, 0, &mut ans, &mut ans_dep);

        ans
    }
}
