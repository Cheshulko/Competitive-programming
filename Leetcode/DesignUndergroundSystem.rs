// https://leetcode.com/problems/design-underground-system

use std::collections::HashMap;

struct UndergroundSystem {
    station_indexes: HashMap<String, usize>,
    travels: HashMap<(usize, usize), Vec<f64>>,
    check_ins: HashMap<i32, (usize, f64)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            station_indexes: HashMap::new(),
            travels: HashMap::new(),
            check_ins: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        let len = self.station_indexes.len();
        let ind = self.station_indexes.entry(station_name).or_insert(len);

        self.check_ins.insert(id, (*ind, t as f64));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let len = self.station_indexes.len();
        let ind = self.station_indexes.entry(station_name).or_insert(len);

        let (start_station_ind, start_t) = self.check_ins.remove(&id).unwrap();

        self.travels
            .entry((start_station_ind, *ind))
            .or_insert(vec![])
            .push((t as f64) - start_t);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let start_ind = self.station_indexes.get(&start_station).unwrap();
        let end_ind = self.station_indexes.get(&end_station).unwrap();

        let v = self.travels.get(&(*start_ind, *end_ind)).unwrap();
        v.iter().sum::<f64>() / (v.len() as f64)
    }
}
