// https://leetcode.com/problems/path-crossing

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        path.chars()
            .fold(
                (HashMap::<(i32, i32), usize>::from([((0, 0), 1)]), (0, 0)),
                |(mut hm, (mut x, mut y)), c| {
                    match c {
                        'N' => y += 1,
                        'S' => y -= 1,
                        'E' => x += 1,
                        'W' => x -= 1,
                        _ => unreachable!(),
                    };
                    *hm.entry((x, y)).or_insert(0) += 1;
                    (hm, (x, y))
                },
            )
            .0
            .values()
            .any(|x| x > &1)
    }
}
