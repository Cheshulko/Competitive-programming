class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        if(nums.size() <= 1) return;
        for(int j = nums.size() - 2; j >= 0; --j){
            if(nums[j + 1] > nums[j]){
                for(int m = nums.size() - 1; m > j; --m){
                    if(nums[m] > nums[j]){
                        swap(nums[m], nums[j]);
                        sort(nums.begin() + j + 1, nums.end());
                        return;
                    }
                }
            }
        }
        sort(nums.begin(), nums.end());
    }
};
