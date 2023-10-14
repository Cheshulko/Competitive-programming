impl Solution {
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        let mut v = vec![];
        let mut cnt = 0;
        let mut ans = vec![];
        for word in words.into_iter() {
            if word == "prev" {
                cnt += 1;
                let x = v
                    .get((v.len() as i32 - cnt) as usize)
                    .unwrap_or(&-1)
                    .clone();
                ans.push(x);
            } else {
                cnt = 0;
                let x = i32::from_str_radix(&word, 10).unwrap();
                v.push(x);
            }
        }

        ans
    }
}
