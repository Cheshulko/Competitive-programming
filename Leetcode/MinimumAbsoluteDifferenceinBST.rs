// https://leetcode.com/problems/minimum-absolute-difference-in-bst

struct Solution {}

impl Solution {
    fn go(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<i32>, Option<i32>) {
        if let Some(cur) = root {
            let mut ans = i32::MAX;
            let cur = cur.borrow();

            let left_res = Solution::go(cur.left.clone());
            let right_res = Solution::go(cur.right.clone());

            ans = ans.min(left_res.0);
            ans = ans.min(right_res.0);

            if let Some(left) = left_res.2 {
                ans = ans.min((cur.val - left).abs());
            }

            if let Some(right) = right_res.1 {
                ans = ans.min((cur.val - right).abs());
            }

            (
                ans,
                left_res.1.or(Some(cur.val)),
                right_res.2.or(Some(cur.val)),
            )
        } else {
            (i32::MAX, None, None)
        }
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::go(root).0
    }
}
