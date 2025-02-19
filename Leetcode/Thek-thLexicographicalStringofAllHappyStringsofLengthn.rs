// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n

struct Solution {}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize - 1;

        let l = [b'a', b'b', b'c'];
        let mut cur = vec![b'a'];

        for i in 1..n {
            for j in 0..3 {
                if cur[i - 1] != l[j] {
                    cur.push(l[j]);
                    break;
                }
            }
        }

        fn add(i: usize, cur: &mut Vec<u8>) -> bool {
            if i == 0 {
                if cur[0] == b'c' {
                    return false;
                } else {
                    cur[0] += 1;

                    return true;
                }
            } else {
                if cur[i] == b'c' {
                    if !add(i - 1, cur) {
                        return false;
                    }

                    if cur[i - 1] == b'a' {
                        cur[i] = b'b';
                    } else {
                        cur[i] = b'a';
                    }

                    return true;
                } else {
                    if cur[i] + 1 == cur[i - 1] {
                        if cur[i] == b'a' {
                            cur[i] += 2;
                        } else {
                            if !add(i - 1, cur) {
                                return false;
                            }

                            if cur[i - 1] == b'a' {
                                cur[i] = b'b';
                            } else {
                                cur[i] = b'a';
                            }
                        }
                    } else {
                        cur[i] += 1;
                    }

                    return true;
                }
            }
        }

        for _ in 0..k {
            if !add(n - 1, &mut cur) {
                return String::new();
            }
        }

        return String::from_utf8(cur).unwrap();
    }
}
