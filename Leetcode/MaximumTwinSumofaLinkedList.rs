// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list

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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        Solution::traverse(&mut head.as_ref(), slow)
    }

    fn traverse(slow: &mut Option<&Box<ListNode>>, fast: &Option<Box<ListNode>>) -> i32 {
        if let Some(fast) = fast {
            let prev_pair_val = Solution::traverse(slow, &fast.as_ref().next);
            let slow_val = slow.as_ref().unwrap().val;

            *slow = slow.as_ref().unwrap().next.as_ref();

            prev_pair_val.max(slow_val + fast.val)
        } else {
            0
        }
    }
}
