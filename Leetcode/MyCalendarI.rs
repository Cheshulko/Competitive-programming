use std::collections::BTreeSet;

struct MyCalendar {
    points: BTreeSet<(usize, usize)>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            points: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let start = start as usize;
        let end = end as usize;
        let mut cnt = 0;

        for &(s, e) in self.points.iter() {
            if s <= start {
                cnt += 1;
            } else {
                break;
            }
            if e <= start {
                cnt -= 1;
            }
        }

        if cnt != 0 {
            return false;
        }

        let mut ok = true;
        for &(s, e) in self.points.iter() {
            if s >= start && s < end {
                ok = false;
                break;
            }
        }

        if ok {
            self.points.insert((start, end));
            return true;
        } else {
            return false;
        }
    }
}
