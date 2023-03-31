// https://leetcode.com/problems/maximum-frequency-stack

use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Eq, PartialOrd, PartialEq, Clone, Copy)]
struct Item {
    freq: i32,
    time: i32,
    value: i32,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.freq, self.time)
            .cmp(&(other.freq, other.time))
            .reverse()
    }
}

struct FreqStack {
    map: BTreeSet<Item>,
    time: HashMap<i32, Vec<i32>>,
    cnt: i32,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            map: BTreeSet::new(),
            time: HashMap::new(),
            cnt: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let freq: i32;

        if let Some(v) = self.map.iter().find(|x| x.value == val).copied() {
            freq = v.freq;
            self.map.remove(&v);
        } else {
            freq = 1;
            self.time.insert(val, Vec::new());
        }

        let r = self.time.get_mut(&val).unwrap();
        r.push(self.cnt);

        self.map.insert(Item {
            freq: freq + 1,
            time: self.cnt,
            value: val,
        });
        self.cnt += 1;
    }

    fn pop(&mut self) -> i32 {
        let v = self.map.iter().take(1).copied().collect::<Vec<_>>()[0];
        let (val, freq) = (v.value, v.freq);
        let r = self.time.get_mut(&val).unwrap();
        r.pop();
        self.map.remove(&v);
        if !r.is_empty() {
            self.map.insert(Item {
                freq: freq - 1,
                time: *r.last().unwrap(),
                value: val,
            });
        }

        val
    }
}
