// https://leetcode.com/problems/robot-collisions

use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
struct State {
    pos: i32,
    hel: i32,
    dir: Direction,
    ind: usize,
}

impl State {
    pub fn is_alive(&self) -> bool {
        self.hel > 0
    }

    pub fn kill(&mut self) {
        self.hel = 0;
    }

    pub fn hit(&mut self) {
        if self.hel > 0 {
            self.hel -= 1;
        }
    }
}

struct Solution {}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let directions = directions.into_bytes();

        let mut arr = (0..positions.len())
            .map(|i| State {
                pos: positions[i],
                hel: healths[i],
                dir: (directions[i] == b'R')
                    .then(|| Direction::Right)
                    .unwrap_or_else(|| Direction::Left),

                ind: i,
            })
            .collect::<Vec<_>>();

        arr.sort_unstable_by(|a, b| a.pos.cmp(&b.pos));

        let mut queue = vec![];

        for mut cur in arr.into_iter() {
            if cur.dir == Direction::Right {
                queue.push(cur);
            } else {
                while let Some(mut last) = queue.pop() {
                    if last.dir == Direction::Right {
                        match last.hel.cmp(&cur.hel) {
                            Ordering::Greater => {
                                last.hit();
                                cur.kill();
                                queue.push(last);

                                break;
                            }
                            Ordering::Less => {
                                cur.hit();
                            }
                            Ordering::Equal => {
                                cur.kill();

                                break;
                            }
                        }
                    } else {
                        queue.push(last);

                        break;
                    }
                }

                if cur.is_alive() {
                    queue.push(cur);
                }
            }
        }

        queue.sort_unstable_by(|a, b| a.ind.cmp(&b.ind));

        queue.into_iter().map(|s| s.hel).collect::<Vec<_>>()
    }
}
