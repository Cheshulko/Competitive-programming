// https://leetcode.com/problems/fruit-into-baskets

struct Solution {}

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        use std::collections::VecDeque;

        let ma = fruits.iter().max().copied().unwrap() as usize;
        let mut have = vec![0; ma + 1];

        let mut take = VecDeque::new();
        let mut dif = 0;

        let mut ans = 0;
        for fruit in fruits.into_iter() {
            let fruit = fruit as usize;

            have[fruit] += 1;
            take.push_back(fruit);

            if have[fruit] == 1 {
                dif += 1;
            }

            while dif > 2 {
                if let Some(fruit) = take.pop_front() {
                    have[fruit] -= 1;
                    if have[fruit] == 0 {
                        dif -= 1;
                    }
                } else {
                    unreachable!()
                }
            }

            ans = ans.max(take.len());
        }

        ans as i32
    }
}
