// https://leetcode.com/problems/meeting-rooms-iii

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings
            .into_iter()
            .map(|meeting| vec![meeting[0] as i64, meeting[1] as i64])
            .collect::<Vec<_>>();
        meetings.sort();

        let n = n as usize;

        let mut rooms = BinaryHeap::<Reverse<(i64, usize)>>::new();
        let mut meetings_at_rooms: Vec<i32> = vec![0; n];
        (0..n).for_each(|i| rooms.push(Reverse((0, i))));

        for meeting in meetings.into_iter() {
            let next_meeting_start_time = meeting[0];

            let free_room = loop {
                let free_room = rooms.peek().unwrap();
                let free_room_ind = free_room.0 .1;
                let free_room_time = free_room.0 .0;

                if free_room_time < next_meeting_start_time {
                    rooms.pop();
                    rooms.push(Reverse((next_meeting_start_time, free_room_ind)));
                } else {
                    break rooms.pop().unwrap();
                }
            };

            let free_room_ind = free_room.0 .1;
            let free_room_time = free_room.0 .0;
            assert!(free_room_time >= next_meeting_start_time);

            meetings_at_rooms[free_room_ind] += 1;

            let meeting_duration = meeting[1] - meeting[0];
            let meetng_end_time = free_room_time + meeting_duration;

            rooms.push(Reverse((meetng_end_time, free_room_ind)));
        }

        meetings_at_rooms
            .into_iter()
            .enumerate()
            .map(|(ind, x)| (x, ind))
            .rev()
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap()
            .1 as i32
    }
}
