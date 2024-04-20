// https://leetcode.com/problems/find-all-groups-of-farmland

struct Solution {}

impl Solution {
    pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = land.len();
        let m = land[0].len();

        let mut ans = vec![];
        for i in 0..n {
            for j in 0..m {
                if land[i][j] == 1 {
                    let mut l = vec![i as i32, j as i32];

                    let (mut i_, mut j_) = (i, j);
                    while i_ < n && land[i_][j] == 1 {
                        j_ = j;
                        while j_ < m && land[i_][j_] == 1 {
                            land[i_][j_] = 0;
                            j_ += 1;
                        }
                        i_ += 1;
                    }

                    l.push(i_ as i32 - 1);
                    l.push(j_ as i32 - 1);

                    ans.push(l);
                }
            }
        }

        ans
    }
}
