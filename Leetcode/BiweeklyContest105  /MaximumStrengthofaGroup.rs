impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let nums = nums;
        let gr = nums
            .iter()
            .filter(|x| x > &&0)
            .map(|x| *x as i64)
            .collect::<Vec<_>>();

        let mut lv = nums
            .iter()
            .filter(|x| x < &&0)
            .map(|x| *x as i64)
            .collect::<Vec<_>>();

        let ans1 = if gr.is_empty() {
            0
        } else {
            gr.iter().product::<i64>()
        };

        lv.sort();
        let mut extra: i64 = 0;
        if lv.len() % 2 == 1 {
            extra = *lv.last().unwrap();
            lv.pop();
        }

        let ans2 = if lv.is_empty() {
            0
        } else {
            lv.iter().product::<i64>()
        };

        let has0 = nums.iter().any(|x| x == &0);

        match (ans1 > 0, ans2 > 0) {
            (true, true) => ans1 * ans2,
            (true, false) => ans1,
            (false, true) => ans2,
            (false, false) => {
                if has0 {
                    0
                } else {
                    gg
                }
            }
        }
    }
}
