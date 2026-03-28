// https://leetcode.com/problems/find-the-string-with-lcp

mod cm {
    use std::mem::swap;

    pub struct DSU {
        parents: Vec<usize>,
        pub ranks: Vec<usize>,
    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            Self {
                parents: (0..size).collect(),
                ranks: vec![1; size],
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if x != self.parents[x] {
                self.parents[x] = self.find(self.parents[x]);
            }

            self.parents[x]
        }

        pub fn same(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            x == y
        }

        pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
            x = self.find(x);
            y = self.find(y);

            if x == y {
                return false;
            }

            if self.ranks[x] < self.ranks[y] {
                swap(&mut y, &mut x);
            }

            self.parents[y] = x;
            self.ranks[x] += self.ranks[y];

            true
        }
    }
}

mod cf {
    pub fn z_fn(s: &[char]) -> Vec<usize> {
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
}

struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();

        let mut dsu = cm::DSU::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                if lcp[i][j] != lcp[j][i] {
                    return String::new();
                }
                if lcp[i][j] > 0 {
                    dsu.union(i, j);
                }
            }
        }

        let mut ans = vec!['#'; n];
        let mut c = b'a';
        let mut gr = vec![b'#'; n];
        for i in 0..n {
            let g = dsu.find(i);
            if gr[g] == b'#' {
                if c > b'z' {
                    return String::new();
                }
                gr[g] = c;
                c += 1;
            }

            ans[i] = gr[g] as char;
        }

        // Validation
        for i in 0..n {
            let mut z_fn = cf::z_fn(&ans[i..]);
            z_fn[0] = n - i;

            for j in 0..z_fn.len() {
                if z_fn[j] as i32 != lcp[i][j + i] {
                    return String::new();
                }
            }
        }

        ans.into_iter().collect()
    }
}
