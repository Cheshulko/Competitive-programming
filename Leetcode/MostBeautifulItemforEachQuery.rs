// https://leetcode.com/problems/most-beautiful-item-for-each-query

struct Solution {}

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<Vec<_>>();
        queries.sort_unstable();

        let mut items = items;
        items.sort_unstable();

        let qn = queries.len();
        let mut ans = vec![0; qn];

        let n = items.len();
        let mut i = 0;
        let mut max_beauty = 0;
        for (q, qi) in queries.into_iter() {
            while i < n && items[i][0] <= q {
                max_beauty = max_beauty.max(items[i][1]);
                i += 1;
            }
            ans[qi] = max_beauty;
        }

        ans
    }
}
