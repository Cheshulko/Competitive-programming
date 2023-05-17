use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BucketStat {
    pub bucket_type: Bucket,
    pub capacity: u8,
    pub value: u8,
}

impl BucketStat {
    fn new(bucket_type: Bucket, capacity: u8) -> Self {
        BucketStat {
            bucket_type,
            capacity,
            value: 0,
        }
    }

    fn full(&mut self) {
        self.value = self.capacity;
    }

    fn free(&mut self) {
        self.value = 0;
    }

    fn add(&mut self, value: u8) {
        self.value += value;
        assert!(self.value <= self.capacity);
    }

    fn remove(&mut self, value: u8) {
        assert!(self.value >= value);
        self.value -= value;
    }

    fn pour_to(&mut self, other: &mut BucketStat) {
        if !other.is_full() {
            let need = other.capacity - other.value;
            let can = self.value.min(need);

            other.add(can);
            self.remove(can);
        }
    }

    fn is_full(&self) -> bool {
        self.value == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.value == 0
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct State {
    pub first: BucketStat,
    pub second: BucketStat,
    pub moves: u8,
}

impl State {
    fn new(moves: u8, first: BucketStat, second: BucketStat) -> Self {
        State {
            moves,
            first,
            second,
        }
    }

    fn is_found(&self, goal: u8) -> Option<BucketStats> {
        if self.first.value == goal {
            return Some(BucketStats {
                moves: self.moves,
                goal_bucket: self.first.bucket_type,
                other_bucket: self.second.value,
            });
        }

        if self.second.value == goal {
            return Some(BucketStats {
                moves: self.moves,
                goal_bucket: self.second.bucket_type,
                other_bucket: self.first.value,
            });
        }

        return None;
    }
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    fn insert_to_used(first_value: u8, second_value: u8, used: &mut HashSet<(u8, u8)>) -> bool {
        let item = (first_value, second_value);
        if !used.contains(&item) {
            used.insert(item);
            true
        } else {
            false
        }
    }

    fn append(
        first: BucketStat,
        second: BucketStat,
        moves: u8,
        start_bucket: &Bucket,
        queue: &mut VecDeque<State>,
        used: &mut HashSet<(u8, u8)>,
    ) {
        if insert_to_used(first.value, second.value, used) {
            if (first.is_full() && second.is_empty() && start_bucket == &second.bucket_type)
                || (second.is_full() && first.is_empty() && start_bucket == &first.bucket_type)
            {
            } else {
                queue.push_back(State::new(moves + 1, first, second));
            }
        }
    }

    let mut used: HashSet<(u8, u8)> = HashSet::new();
    let mut queue: VecDeque<State> = VecDeque::new();

    let (mut init_first, mut init_second) = (
        BucketStat::new(Bucket::One, capacity_1),
        BucketStat::new(Bucket::Two, capacity_2),
    );

    queue.push_back(match start_bucket {
        Bucket::One => {
            init_first.full();
            State::new(1, init_first, init_second)
        }
        Bucket::Two => {
            init_second.full();
            State::new(1, init_first, init_second)
        }
    });

    while let Some(current_state) = queue.pop_front() {
        if let Some(result) = current_state.is_found(goal) {
            return Some(result);
        }

        let _ = insert_to_used(
            current_state.first.value,
            current_state.second.value,
            &mut used,
        );

        let bucket_transforms: [Box<dyn Fn(BucketStat, BucketStat) -> (BucketStat, BucketStat)>;
            6] = [
            Box::new(|mut first, mut second| {
                first.pour_to(&mut second);
                (first, second)
            }),
            Box::new(|mut first, mut second| {
                second.pour_to(&mut first);
                (first, second)
            }),
            Box::new(|mut first, second| {
                first.full();
                (first, second)
            }),
            Box::new(|mut first, second| {
                first.free();
                (first, second)
            }),
            Box::new(|first, mut second| {
                second.full();
                (first, second)
            }),
            Box::new(|first, mut second| {
                second.free();
                (first, second)
            }),
        ];

        bucket_transforms.iter().for_each(|transform| {
            let (first, second) =
                transform(current_state.first.clone(), current_state.second.clone());

            append(
                first,
                second,
                current_state.moves,
                start_bucket,
                &mut queue,
                &mut used,
            );
        });
    }

    None
}
