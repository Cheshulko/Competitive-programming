impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut ans = 2.min(s.len() as i32);
        let mut v: Vec<i32> = vec![];
        let mut cnt = 0;
        let mut have = false;

        for j in 1..s.len() {
            if s.chars().nth(j - 1) == s.chars().nth(j) {
                have = true;
                v.push(cnt);
                cnt = 0;
                v.push(-1);
            } else {
                cnt += 1;
            }
        }
        if have {
            v.push(cnt);
        } else {
            v.push(cnt + 1);
        }

        for i in 0..v.len() {
            let x = v[i];
            if x == -1 {
                let mut lc = 0;
                if i > 0 && v[i - 1] != -1 {
                    lc += v[i - 1];
                }
                if i + 1 < v.len() && v[i + 1] != -1 {
                    lc += v[i + 1];
                }
                ans = ans.max(lc + 2)
            } else {
                ans = ans.max(x);
            }
        }

        ans as i32
    }
}
