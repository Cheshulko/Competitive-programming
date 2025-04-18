// https://leetcode.com/problems/count-and-say

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        fn solve(mut a: Vec<u8>) -> Vec<u8> {
            let mut b = vec![];

            a.push(b'#');
            let mut cnt = 0;
            let mut cur = a[0];
            for c in a.into_iter() {
                if c == cur {
                    cnt += 1;
                } else {
                    b.extend(format!("{}", cnt).into_bytes());
                    b.push(cur);
                    cur = c;
                    cnt = 1;
                }
            }

            b
        }

        let mut cur = vec![b'1'];
        for _ in 1..n {
            cur = solve(cur);
        }

        String::from_utf8(cur).unwrap()
    }
}
