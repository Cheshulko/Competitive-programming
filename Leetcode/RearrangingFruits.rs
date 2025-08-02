// https://leetcode.com/problems/rearranging-fruits

struct Solution {}

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        use std::cmp::Ordering;
        use std::collections::{BTreeSet, HashMap, VecDeque};

        let mut unique = BTreeSet::new();
        let mut cnt1 = HashMap::<i32, usize>::new();
        let mut cnt2 = HashMap::<i32, usize>::new();

        for &fruit in basket1.iter() {
            unique.insert(fruit);
            *cnt1.entry(fruit).or_default() += 1;
        }

        for &fruit in basket2.iter() {
            unique.insert(fruit);
            *cnt2.entry(fruit).or_default() += 1;
        }

        let mut from1 = VecDeque::new();
        let mut from2 = VecDeque::new();
        let best = unique.first().copied().unwrap();
        for &fruit in unique.iter() {
            let in1 = cnt1.get(&fruit).copied().unwrap_or_default();
            let in2 = cnt2.get(&fruit).copied().unwrap_or_default();

            if (in1 + in2) % 2 == 1 {
                return -1;
            }

            let dif = in1.abs_diff(in2) / 2;
            match in1.cmp(&in2) {
                Ordering::Less => from2.extend([fruit].repeat(dif)),
                Ordering::Greater => from1.extend([fruit].repeat(dif)),
                Ordering::Equal => {}
            }
        }

        let mut ans = 0;
        while let (Some(&fruit1), Some(&fruit2)) = (from1.front(), from2.front()) {
            if fruit1.min(fruit2) > 2 * best {
                from1.push_front(best);
                from2.push_front(best);

                continue;
            }
            if fruit1 < fruit2 {
                ans += fruit1 as i64;
                from1.pop_front();
                from2.pop_back();
            } else {
                ans += fruit2 as i64;
                from2.pop_front();
                from1.pop_back();
            }
        }

        ans
    }
}
