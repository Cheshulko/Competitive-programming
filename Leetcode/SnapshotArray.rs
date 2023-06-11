use std::collections::BTreeMap;

struct SnapshotArray {
    m: Vec<BTreeMap<i32, i32>>,
    cur: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            m: vec![BTreeMap::new(); length as usize],
            cur: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.m[index as usize].insert(self.cur, val);
    }

    fn snap(&mut self) -> i32 {
        self.cur += 1;
        self.cur - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        self.m[index as usize]
            .range(..=snap_id)
            .next_back()
            .map(|(_, v)| v)
            .unwrap_or(&0)
            .clone()
    }
}
