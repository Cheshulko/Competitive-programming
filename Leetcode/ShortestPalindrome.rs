impl Solution {
    fn z_fn(s: &[u8]) -> Vec<usize> {
        let n = s.len();
        let mut z = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if r >= i {
                z[i] = z[i - l].min(r - i + 1);
            }
            while z[i] + i < n && s[z[i]] == s[z[i] + i] {
                z[i] += 1;
            }
            if r < i + z[i] - 1 {
                l = i;
                r = i + z[i] - 1;
            }
        }

        z
    }
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();

        let mut s = s.into_bytes();
        let s2 = s.clone();
        let mut ans = s.clone();
        ans.reverse();
        s.push(b'#');
        s.extend(s2.into_iter().rev());
        let z_arr = Solution::z_fn(&s);

        let mut pos = 0;
        for i in (n + 1)..s.len() {
            if z_arr[i] > pos && z_arr[i] + i == s.len() {
                pos = z_arr[i];
                break;
            }
        }

        for i in pos..n {
            ans.push(s[i]);
        }

        String::from_utf8(ans).unwrap()
    }
}
