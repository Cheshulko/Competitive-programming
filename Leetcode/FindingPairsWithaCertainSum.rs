use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    nums2_cnt: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut nums2_cnt = HashMap::new();

        for &num in nums2.iter() {
            *nums2_cnt.entry(num).or_default() += 1;
        }

        FindSumPairs {
            nums1,
            nums2,
            nums2_cnt,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;

        let old = self.nums2[index];
        *self.nums2_cnt.get_mut(&old).unwrap() -= 1;

        self.nums2[index] += val;
        let new = self.nums2[index];
        *self.nums2_cnt.entry(new).or_default() += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        let mut ans = 0;

        for &a in self.nums1.iter() {
            ans += self.nums2_cnt.get(&(tot - a)).unwrap_or(&0);
        }

        ans
    }
}
