// https://leetcode.com/problems/most-beautiful-item-for-each-query

struct Solution {}

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let n = items.len();
        items.sort_unstable();

        let mut pref_max = vec![0; n + 1];
        for i in 0..n {
            pref_max[i + 1] = pref_max[i].max(items[i][1]);
        }

        queries
            .into_iter()
            .map(|q| {
                let p = items.partition_point(|x| x[0] <= q);
                if p == 0 {
                    0
                } else {
                    pref_max[p]
                }
            })
            .collect()
    }
}
