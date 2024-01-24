// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree

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
    fn go(root: Option<Rc<RefCell<TreeNode>>>, cnt: &mut [usize]) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            let left = root.left.clone();
            let right = root.right.clone();

            cnt[root.val as usize] += 1;
            let ans = if left.is_none() && right.is_none() {
                (cnt.iter().filter(|x| **x % 2 == 1).count() <= 1) as i32
            } else {
                Solution::go(left, cnt) + Solution::go(right, cnt)
            };
            cnt[root.val as usize] -= 1;
            ans
        } else {
            0
        }
    }

    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cnt = [0; 10];
        Solution::go(root, &mut cnt)
    }
}
