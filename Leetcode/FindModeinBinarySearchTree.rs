// https://leetcode.com/problems/find-mode-in-binary-search-tree

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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, cnt: &mut Vec<i32>) {
        if let Some(root) = root {
            let root = root.borrow();
            cnt[100_000 + root.val as usize] += 1;
            Solution::dfs(root.left.clone(), cnt);
            Solution::dfs(root.right.clone(), cnt);
        }
    }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut cnt = vec![0; 2 * 100_000 + 1];
        Solution::dfs(root, &mut cnt);
        let mx = *cnt.iter().max().unwrap();
        cnt.into_iter()
            .enumerate()
            .filter_map(|(ind, x)| {
                if x == mx {
                    Some(ind as i32 - 100_000)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
