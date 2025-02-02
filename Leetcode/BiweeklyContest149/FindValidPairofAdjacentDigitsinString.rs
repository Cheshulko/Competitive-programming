impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let mut cnt = vec![0; 10];
        for c in s.as_bytes() {
            let c = (c - b'0') as usize;
            cnt[c] += 1;
        }

        let s = s.into_bytes().into_iter().collect::<Vec<_>>();
        for w in s.windows(2) {
            let (c1, c2) = (w[0], w[1]);

            let (cc1, cc2) = ((c1 - b'0') as usize, (c2 - b'0') as usize);

            if c1 != c2 && cnt[cc1] == cc1 && cnt[cc2] == cc2 {
                let ans = vec![c1, c2];
                return String::from_utf8(ans).unwrap();
            }
        }

        return String::new();
    }
}
