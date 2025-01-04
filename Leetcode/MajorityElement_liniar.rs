struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count_on_stack = 0;
        let mut element = 0;

        for num in nums {
            if count_on_stack == 0 {
                count_on_stack += 1;
                element = num;
            } else if element == num {
                count_on_stack += 1;
            } else
            /* element != num */
            {
                count_on_stack -= 1;
            }
        }

        element
    }
}
