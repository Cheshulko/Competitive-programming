// https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero

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

struct Solution {}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dsu = cm::DSU::new(n);
        let ma = nums.iter().max().copied().unwrap() as usize;

        let mut num_indx = vec![vec![]; ma + 1];
        for (i, &num) in nums.iter().enumerate() {
            num_indx[num as usize].push(i);
        }

        let mut ans = 0;
        for indx in num_indx.iter().rev() {
            if indx.is_empty() {
                continue;
            }

            let mut prev = usize::MAX;
            for &i in indx.iter() {
                if i > 0 && nums[i - 1] >= nums[i] {
                    dsu.union(i, i - 1);
                }
                if i < n - 1 && nums[i + 1] >= nums[i] {
                    dsu.union(i, i + 1);
                }

                let g = dsu.find(i);
                if prev != usize::MAX {
                    let gp = dsu.find(prev);
                    if gp != g {
                        ans += 1;
                    }
                } else {
                    ans += 1;
                }
                prev = i;
            }
        }

        let mi = nums.iter().min().copied().unwrap();
        if mi == 0 {
            ans -= 1;
        }

        ans
    }
}
