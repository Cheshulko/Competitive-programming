// https://leetcode.com/problems/seat-reservation-manager

use std::collections::BinaryHeap;
use std::iter::FromIterator;

struct SeatManager {
    seats: BinaryHeap<i32>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            seats: BinaryHeap::<i32>::from_iter(-n..0),
        }
    }

    fn reserve(&mut self) -> i32 {
        -self.seats.pop().unwrap()
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.seats.push(-seat_number)
    }
}
