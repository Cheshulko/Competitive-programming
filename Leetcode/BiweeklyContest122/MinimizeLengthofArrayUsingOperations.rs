impl Solution {
    pub fn minimum_array_length(mut nums: Vec<i32>) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while a != 0 {
                if a < b {
                    std::mem::swap(&mut a, &mut b);
                }
                a %= b;
            }
            b
        }

        let mut x = nums[0];
        for i in 1..nums.len() {
            x = gcd(x, nums[i]);
        }

        nums.sort();
        let f = nums[0];
        let cnt = nums.iter().filter(|x| x == &&f).count() as i32;

        if nums.iter().filter(|u| u == &&x).count() == 0 {
            1
        } else {
            cnt / 2 + cnt % 2
        }
    }
}
