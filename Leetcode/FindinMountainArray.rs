// https://leetcode.com/problems/find-in-mountain-array

/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

struct Solution {}

impl Solution {
    fn ternary_search(n: i32, mountainArr: &MountainArray) -> i32 {
        let mut l = 0;
        let mut r = n - 1;

        while r - l >= 3 {
            let m1 = l + (r - l) / 3;
            let m2 = r - (r - l) / 3;

            if mountainArr.get(m1) < mountainArr.get(m2) {
                l = m1;
            } else {
                r = m2;
            }
        }

        let mut mx_ind = 0;
        let mut mx = 0;

        (l..=r).for_each(|x| {
            let y = mountainArr.get(x);
            if y > mx {
                mx = y;
                mx_ind = x;
            }
        });

        mx_ind
    }

    fn binary_search<F>(
        mut l: i32,
        mut r: i32,
        target: i32,
        mountainArr: &MountainArray,
        mut compare: F,
    ) -> i32
    where
        F: FnMut(&i32, &i32) -> bool,
    {
        while r - l > 1 {
            let m = (r + l) / 2;
            let m_value = mountainArr.get(m);
            if compare(&target, &m_value) {
                r = m;
            } else {
                l = m;
            }
        }

        if mountainArr.get(l) == target {
            l
        } else {
            -1
        }
    }

    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let n = mountainArr.length();
        let mx_ind = Solution::ternary_search(n, mountainArr);

        let left_part = Solution::binary_search(0, mx_ind + 1, target, mountainArr, |a, b| a < b);
        if left_part != -1 {
            return left_part;
        }

        let right_part =
            Solution::binary_search(mx_ind, n as i32, target, mountainArr, |a, b| a > b);
        if right_part != -1 {
            return right_part;
        }

        -1
    }
}
