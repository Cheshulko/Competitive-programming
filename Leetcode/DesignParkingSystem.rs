// https://leetcode.com/problems/design-parking-system

struct ParkingSystem {
    cnt: [i32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            cnt: [big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index = (car_type - 1) as usize;
        if self.cnt[index] > 0 {
            self.cnt[index] -= 1;
            return true;
        } else {
            return false;
        }
    }
}
