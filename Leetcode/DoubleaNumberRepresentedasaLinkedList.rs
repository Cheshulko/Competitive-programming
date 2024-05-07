// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list

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
    pub fn double_it_(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        if let Some(mut head_) = head {
            let (next, add) = Solution::double_it_(head_.next);
            head_.val = head_.val * 2 + add;
            head_.next = next;

            let cur = head_.val / 10;
            head_.val %= 10;

            (Some(head_), cur)
        } else {
            (head, 0)
        }
    }

    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (head, add) = Solution::double_it_(head.clone());
        if add != 0 {
            Some(Box::new(ListNode {
                val: add,
                next: head,
            }))
        } else {
            head
        }
    }
}
