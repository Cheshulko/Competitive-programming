// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii

struct Solution {}

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const V: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        let k = k as usize;
        let n = word.len();
        let word = word.chars().collect::<Vec<_>>();

        let mut consonants = vec![0; n + 1];
        consonants[0] = 1;
        let mut consonants_cnt = 0;

        let mut vowels = vec![0; 26];
        let mut vowels_unique_cnt = 0;

        let mut consonant_pos = vec![-1];
        for i in 0..n {
            if !V.contains(&word[i]) {
                consonant_pos.push(i as i64);
            }
        }

        let mut ans = 0;

        let mut l = 0;
        for r in 0..n {
            if V.contains(&word[r]) {
                let v = (word[r] as u8 - b'a') as usize;
                if vowels[v] == 0 {
                    vowels_unique_cnt += 1;
                }
                vowels[v] += 1;
            } else {
                consonants_cnt += 1;
            }

            while l <= r && consonants_cnt > k {
                if !V.contains(&word[l]) {
                    consonants_cnt -= 1;
                } else {
                    let v = (word[l] as u8 - b'a') as usize;
                    vowels[v] -= 1;
                    if vowels[v] == 0 {
                        vowels_unique_cnt -= 1;
                    }
                }
                l += 1;
            }

            while l <= r && V.contains(&word[l]) {
                let v = (word[l] as u8 - b'a') as usize;
                if vowels[v] <= 1 {
                    break;
                }

                vowels[v] -= 1;
                l += 1;
            }

            if consonants_cnt == k && vowels_unique_cnt == V.len() {
                let l_prev = consonant_pos.partition_point(|&x| x < l as i64) - 1;
                let l_prev = consonant_pos[l_prev];
                ans += l as i64 - l_prev;
            }
        }

        ans
    }
}
