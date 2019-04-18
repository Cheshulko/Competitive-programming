class Solution {
   public:
    bool checkPossibility(vector<int>& nums) {
        if (nums.empty()) return true;

        bool can_dec = true;
        bool can_inc = true;

        for (int i = 0; i < nums.size() - 1; ++i) {
            if (nums[i] > nums[i + 1]) {
                if (i - 1 >= 0 && nums[i - 1] > nums[i + 1]) can_dec = false;

                if (i + 2 < nums.size() && nums[i + 2] < nums[i])
                    can_inc = false;

                if (can_dec) {
                    nums[i] = nums[i + 1];
                } else if (can_inc) {
                    nums[i + 1] = nums[i];
                } else
                    return false;
                break;
            }
        }
        for (int i = 0; i < nums.size() - 1; ++i)
            if (nums[i] > nums[i + 1]) return false;

        return true;
    }
};