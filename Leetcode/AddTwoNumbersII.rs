// https://leetcode.com/problems/add-two-numbers-ii

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
    fn dfs(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        d1: i32,
        d2: i32,
    ) -> (Option<Box<ListNode>>, i32) {
        if d1 > 0 {
            let mut cur = Box::new(ListNode::new(0));
            let l1 = l1.unwrap();
            let (next, up) = Solution::dfs(l1.next, l2, d1 - 1, d2);
            cur.next = next;
            cur.val = (l1.val + up) % 10;

            return (Some(cur), (l1.val + up) / 10);
        }

        if d2 > 0 {
            let mut cur = Box::new(ListNode::new(0));
            let l2 = l2.unwrap();
            let (next, up) = Solution::dfs(l1, l2.next, d1, d2 - 1);
            cur.next = next;
            cur.val = (l2.val + up) % 10;

            return (Some(cur), (l2.val + up) / 10);
        }

        if let (Some(l1), Some(l2)) = (l1, l2) {
            let mut cur = Box::new(ListNode::new(0));
            let (next, up) = Solution::dfs(l1.next, l2.next, d1, d2);
            cur.next = next;
            cur.val = (l1.val + l2.val + up) % 10;

            return (Some(cur), (l1.val + l2.val + up) / 10);
        }

        (None, 0)
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur_l1 = l1.clone();
        let mut cur_l2 = l2.clone();

        while let (Some(ref _cur_l1), Some(ref _cur_l2)) = (&cur_l1, &cur_l2) {
            cur_l1 = _cur_l1.next.clone();
            cur_l2 = _cur_l2.next.clone();
        }

        let mut d1 = 0;
        let mut d2 = 0;

        while let Some(_cur_l1) = cur_l1 {
            d1 += 1;
            cur_l1 = _cur_l1.next.clone();
        }

        while let Some(_cur_l2) = cur_l2 {
            d2 += 1;
            cur_l2 = _cur_l2.next.clone();
        }

        let (mut head, up) = Solution::dfs(l1, l2, d1, d2);
        if up > 0 {
            let mut cur = Box::new(ListNode::new(up));
            cur.next = head;
            head = Some(cur);
        }

        head
    }
}
