// https://leetcode.com/problems/minimum-number-of-flips-to-convert-binary-matrix-to-zero-matrix

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution {}

impl Solution {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    const MAX: usize = 1 << 3;
}

impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        Solution::bfs(
            mat.len(),
            mat.get(0).unwrap().len(),
            vec![0; mat.len()],
            Solution::to_bits_rep(&mat),
        )
    }

    fn to_bits_rep(mat: &Vec<Vec<i32>>) -> Vec<usize> {
        fn get_value(row: &Vec<i32>, index: usize) -> usize {
            let value: &i32 = row.get(index).unwrap_or(&0);
            *value as usize
        }

        fn get_row_value(mat: &Vec<Vec<i32>>, index: usize) -> usize {
            match mat.get(index) {
                Some(row) => {
                    (0..row.len()).fold(0, |sum, index| sum + (1 << index) * get_value(row, index))
                }
                None => 0,
            }
        }

        vec![0; mat.len()]
            .iter_mut()
            .enumerate()
            .map(|(index, _)| get_row_value(mat, index))
            .collect::<Vec<usize>>()
    }

    fn can(i: i32, j: i32, n: usize, m: usize) -> bool {
        let ni32 = n as i32;
        let mi32 = m as i32;

        i >= 0 && i < ni32 && j >= 0 && j < mi32
    }

    fn inverse(bits: &mut Vec<usize>, i: usize, j: usize) {
        *bits.get_mut(i).unwrap() ^= 1 << j;
    }

    fn bfs(n: usize, m: usize, initial: Vec<usize>, target: Vec<usize>) -> i32 {
        let mut queue: VecDeque<(Vec<usize>, usize)> = VecDeque::new();
        let mut used: HashSet<Vec<usize>> = HashSet::new();

        queue.push_back((initial, 0));

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            used.insert(current.0.to_vec());

            if &current.0 == &target {
                return current.1 as i32;
            }

            for i in 0..n {
                for j in 0..m {
                    let mut current_bits = current.0.to_vec();
                    for (dx, dy) in &Solution::DIRS {
                        let to_i = i as i32 + dx;
                        let to_j = j as i32 + dy;
                        let is_can = Solution::can(to_i, to_j, n, m);
                        if is_can {
                            Solution::inverse(&mut current_bits, to_i as usize, to_j as usize);
                        }
                    }
                    Solution::inverse(&mut current_bits, i, j);
                    let is_used = used.get(&current_bits).is_some();

                    if !is_used {
                        queue.push_back((current_bits, current.1 + 1));
                    }
                }
            }
        }
        -1
    }
}
