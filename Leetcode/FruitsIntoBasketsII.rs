// https://leetcode.com/problems/fruits-into-baskets-ii

struct Solution {}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut free = vec![true; baskets.len()];

        let n = fruits.len();
        let mut can = 0;
        'out: for fruit in fruits.into_iter() {
            for (i, &basket) in baskets.iter().enumerate() {
                if !free[i] {
                    continue;
                }

                if fruit <= basket {
                    can += 1;
                    free[i] = false;

                    continue 'out;
                }
            }
        }

        (n - can) as i32
    }
}
