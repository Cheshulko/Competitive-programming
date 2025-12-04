// https://leetcode.com/problems/count-collisions-on-a-road

struct Solution {}

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut q = vec![];
        let mut ans = 0;
        for c in directions.chars() {
            match c {
                'L' => {
                    if let Some(x) = q.pop() {
                        ans += 1 + x;
                        while let Some(x) = q.pop() {
                            ans += x;
                        }
                        q.push(0);
                    }
                }
                'R' => {
                    q.push(1);
                }
                'S' => {
                    while let Some(x) = q.pop() {
                        ans += x;
                    }
                    q.push(0);
                }
                _ => unreachable!(),
            }
        }

        ans
    }
}
