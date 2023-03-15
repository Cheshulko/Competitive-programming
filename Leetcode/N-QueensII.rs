// https://leetcode.com/problems/n-queens-ii/description/

impl Solution {
    pub fn go(v: &mut Vec<(i32, i32)>, cnt: &mut i32, ans: &mut i32, n: &i32) {
        if cnt == n {
            *ans += 1;
            return;
        }
        let i = *cnt;
        for j in 0..*n {
            let mut can = true;

            for vc in v.iter() {
                if vc.1 == j {
                    can = false;
                    break;
                }
                if (vc.0 - i).abs() == (vc.1 - j).abs() {
                    can = false;
                    break;
                }
            }
            if can {
                v.push((i, j));
                *cnt += 1;
                Solution::go(v, cnt, ans, n);
                v.pop();
                *cnt -= 1;
            }
        }
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut v: Vec<(i32, i32)> = vec![];
        let mut ans = 0;
        let mut cnt = 0;
        Solution::go(&mut v, &mut cnt, &mut ans, &n);

        ans
    }
}
