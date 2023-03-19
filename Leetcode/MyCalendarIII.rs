// https://leetcode.com/problems/my-calendar-iii/description/

use std::collections::{BTreeMap, BTreeSet};

struct MyCalendarThree {
    mapper: BTreeMap<i32, usize>,
    sorted: BTreeSet<i32>,
    ranges: Vec<(i32, i32)>,
    cnt: [i32; 802],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            mapper: BTreeMap::new(),
            sorted: BTreeSet::new(),
            ranges: Vec::new(),
            cnt: [0; 802],
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.ranges.push((start_time, end_time));

        self.sorted.insert(start_time);
        self.sorted.insert(end_time);

        self.cnt = [0; 802];
        self.mapper.clear();

        let mut ind_start_mapper: i32 = -1;
        let mut ind_end_mapper: i32 = -1;
        for (ind, v) in self.sorted.iter().enumerate() {
            self.mapper.insert(*v, ind as usize);
        }

        for (st, en) in &self.ranges {
            let st_mapped = *self.mapper.get(st).unwrap();
            let en_mapped = *self.mapper.get(en).unwrap();

            for up in st_mapped..en_mapped {
                self.cnt[up] += 1;
            }
        }

        let mut ans = 0;

        for i in 0..self.sorted.len() {
            ans = std::cmp::max(ans, self.cnt[i]);
        }

        ans
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */
fn main() {
    let mut myCalendarThree = MyCalendarThree::new();
    println!("{}", myCalendarThree.book(10, 20)); // return 1
    println!("{}", myCalendarThree.book(50, 60)); // return 1
    println!("{}", myCalendarThree.book(10, 40)); // return 2
    println!("{}", myCalendarThree.book(5, 15)); // return 3
    println!("{}", myCalendarThree.book(5, 10)); // return 3
    println!("{}", myCalendarThree.book(25, 55)); // return 3
}
