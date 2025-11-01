// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

struct Solution {}

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let ma = nums.iter().max().copied().unwrap_or(0) as usize;
        let nums = nums.into_iter().fold(vec![false; ma + 1], |mut v, x| {
            v[x as usize] = true;
            v
        });

        fn solve(nums: &[bool], head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut head) = head {
                let next = head.next.take();
                let next = solve(nums, next);

                let val = head.val as usize;
                if val >= nums.len() || !nums[val] {
                    head.next = next;

                    Some(head)
                } else {
                    next
                }
            } else {
                None
            }
        }

        solve(&nums, head)
    }
}
