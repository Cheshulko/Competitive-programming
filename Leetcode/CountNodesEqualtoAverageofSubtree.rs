// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree

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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, usize) {
        if let Some(root) = root {
            let root = root.borrow();
            let (sum_left, count_left, ans_left) = Solution::dfs(root.left.clone());
            let (sum_right, count_right, ans_right) = Solution::dfs(root.right.clone());

            let mut ans = ans_left + ans_right;
            let sum = sum_left + sum_right + root.val;
            let count = count_left + count_right + 1;
            if root.val == sum / count {
                ans += 1;
            }

            (sum, count, ans)
        } else {
            (0, 0, 0)
        }
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ans = Solution::dfs(root);
        ans.2 as i32
    }
}
