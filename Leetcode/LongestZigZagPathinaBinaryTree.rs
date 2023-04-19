// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn dfs(cur: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
        if let Some(cur) = cur {
            let to_left = Solution::dfs(cur.borrow_mut().left.clone(), ans);
            let to_right = Solution::dfs(cur.borrow_mut().right.clone(), ans);

            *ans = *[*ans, to_left.0, to_left.1, to_right.0, to_right.1]
                .iter()
                .max()
                .unwrap();
            (to_left.1 + 1, to_right.0 + 1)
        } else {
            (0, 0)
        }
    }
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let x = Solution::dfs(root, &mut ans);
        *[x.0, x.1, ans].iter().max().unwrap() - 1
    }
}
