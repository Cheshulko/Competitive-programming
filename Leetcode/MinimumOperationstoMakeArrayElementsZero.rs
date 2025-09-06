// https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero

struct Solution {}

pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
    fn get(e: i64) -> [i64; 32] {
        let mut cnt = [0; 32];
        let mut cur = 0;
        let mut v = 1;
        let mut p = 0;

        while v - 1 <= e {
            cnt[p] += v - 1 - cur;
            cur = v - 1;
            p += 1;
            v *= 4;
        }

        cnt[p] += e - cur;

        cnt
    }

    let mut ans = 0;
    for query in queries {
        let (s, e) = (query[0] as i64, query[1] as i64);
        let e = get(e);
        let s = get(s - 1);
        let mut a = [0; 32];
        for i in 0..32 {
            a[i] = e[i] - s[i];
        }

        for i in (1..32).rev() {
            ans += a[i] / 2;
            a[i - 1] += a[i] / 2 * 2;

            if a[i] % 2 == 1 {
                ans += 1;
                if i > 1 {
                    a[i - 2] += 1;
                }
            }
        }
    }

    ans
}
