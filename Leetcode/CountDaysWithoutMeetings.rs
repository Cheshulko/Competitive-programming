// https://leetcode.com/problems/count-days-without-meetings

struct Solution {}

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable();
        meetings.push(vec![days + 1, days + 1]);

        let mut cnt = 0;
        let mut end = 0;
        for meeting in meetings {
            cnt += (meeting[0] - end - 1).max(0);
            end = end.max(meeting[1]);
        }

        cnt
    }
}
