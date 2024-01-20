impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        fn bits(mut x: i32) -> usize {
            let mut cnt = 0;
            while x > 0 {
                cnt += 1;
                x = x & (x - 1);
            }
            cnt
        }

        let n = nums.len();
        let mut bt = vec![0; n];
        for (ind, x) in nums.iter().enumerate() {
            bt[ind] = bits(*x);
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..(n - 1) {
                    if nums[j] > nums[j + 1] && bt[j] == bt[j + 1] {
                        let x = nums[j];
                        nums[j] = nums[j + 1];
                        nums[j + 1] = x;
                    }
                }
            }
        }

        let mut nums2 = nums.clone();
        nums2.sort();

        nums == nums2
    }
}
