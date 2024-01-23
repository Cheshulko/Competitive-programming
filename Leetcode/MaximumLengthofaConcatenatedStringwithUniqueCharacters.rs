// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters

struct Solution {}

impl Solution {
    fn can_concat(x: &Vec<bool>, y: &Vec<bool>) -> bool {
        !x.iter().zip(y.iter()).any(|(a, b)| *a && *b)
    }

    fn concat(x: &mut Vec<bool>, y: &Vec<bool>) -> usize {
        for (ind, y_) in y.iter().enumerate() {
            y_.then(|| x[ind] = true);
        }
        x.iter().filter(|x| **x).count()
    }

    fn remove(x: &mut Vec<bool>, y: &Vec<bool>) {
        for (ind, y_) in y.iter().enumerate() {
            y_.then(|| x[ind] = false);
        }
    }

    fn dfs<'a>(cur: usize, chars: &mut Vec<bool>, arr: &Vec<Vec<bool>>) -> usize {
        let mut ans = 0;
        for to in (cur + 1)..arr.len() {
            if Solution::can_concat(chars, &arr[to]) {
                ans = ans.max(Solution::concat(chars, &arr[to]));
                ans = ans.max(Solution::dfs(to, chars, arr));
                Solution::remove(chars, &arr[to]);
            }
        }
        ans
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut arr = arr
            .iter()
            .filter_map(|s| {
                s.as_bytes().iter().try_fold(vec![false; 26], |mut v, c| {
                    (!v[(*c - b'a') as usize]).then(|| {
                        v[(*c - b'a') as usize] = true;
                        v
                    })
                })
            })
            .collect::<Vec<_>>();
        arr.push(vec![]);

        let mut ans = 0;
        for i in 0..arr.len() {
            let mut x = arr[i].clone();
            ans = ans.max(Solution::dfs(i, &mut x, &arr));
        }

        ans as i32
    }
}
