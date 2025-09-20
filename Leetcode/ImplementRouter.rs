// https://leetcode.com/problems/implement-router

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

type Packet = [i32; 3];
type PacketCounted = [i32; 4];

struct Router {
    queue: BTreeMap<Packet, i32>,
    to_dist: HashMap<i32, VecDeque<i32>>,
    limit: usize,

    queue_because_of_stupid_description: BTreeSet<PacketCounted>,
    counter: i32,
}

impl Router {
    fn new(memoryLimit: i32) -> Self {
        let limit = memoryLimit as usize;

        Self {
            queue: BTreeMap::new(),
            to_dist: HashMap::new(),
            limit,
            queue_because_of_stupid_description: BTreeSet::new(),
            counter: 0,
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = [timestamp, source, destination];

        if self.queue.contains_key(&packet) {
            return false;
        }

        if self.queue_because_of_stupid_description.len() == self.limit {
            let [timestamp, _, source, destination] = self
                .queue_because_of_stupid_description
                .pop_first()
                .unwrap();

            self.queue.remove(&[timestamp, source, destination]);

            let timestamp_ = self
                .to_dist
                .get_mut(&destination)
                .unwrap()
                .pop_front()
                .unwrap();

            assert!(timestamp == timestamp_);
        }

        self.counter += 1;
        self.queue.insert(packet, self.counter);

        self.queue_because_of_stupid_description.insert([
            timestamp,
            self.counter,
            source,
            destination,
        ]);

        self.to_dist
            .entry(destination)
            .or_default()
            .push_back(timestamp);

        return true;
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some([timestamp, _, source, destination]) =
            self.queue_because_of_stupid_description.pop_first()
        {
            self.queue.remove(&[timestamp, source, destination]);
            self.to_dist.get_mut(&destination).unwrap().pop_front();

            return vec![source, destination, timestamp];
        }

        return vec![];
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(timestamps) = self.to_dist.get(&destination) {
            let start = timestamps.partition_point(|&ts| ts < start_time);
            let end = timestamps.partition_point(|&ts| ts <= end_time);

            return (end - start) as i32;
        }

        return 0;
    }
}
