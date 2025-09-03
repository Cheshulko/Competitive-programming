// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii

struct Solution {}

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::BTreeMap;

        let points = points
            .into_iter()
            .enumerate()
            .map(|(i, point)| (point[0], point[1]))
            .collect::<Vec<_>>();

        let mut points_by_x = BTreeMap::<i32, Vec<&(i32, i32)>>::new();
        for point in points.iter() {
            points_by_x.entry(point.0).or_default().push(point);
        }
        for points_line in points_by_x.values_mut() {
            points_line.sort_unstable_by_key(|point| point.1);
        }

        let mut ans = 0;
        for points_line in points_by_x.values() {
            for &&(x1, y1) in points_line.iter() {
                let mut lower = i32::MIN;
                let mut it = points_by_x.range(x1..);
                while let Some(line_to) = it.next() {
                    assert!(x1 <= *line_to.0);

                    let p = if x1 == *line_to.0 {
                        line_to.1.partition_point(|v| v.1 < y1)
                    } else {
                        line_to.1.partition_point(|v| v.1 <= y1)
                    };

                    if p != 0 {
                        let &(_, y2) = line_to.1[p - 1];
                        if y2 > lower {
                            lower = y2;
                            ans += 1;
                        }
                    }
                }
            }
        }

        ans
    }
}
