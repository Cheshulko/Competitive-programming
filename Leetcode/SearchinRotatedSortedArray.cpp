class Solution {
public:
    int search(vector<int>& nums, int target) {
        int l = 0, r = nums.size() - 1;
        while(r - l > 1){
            int m = (l + r) / 2;
            if(nums[m] <= nums[r]){
                if(nums[m] <= target && target <= nums[r]) l = m;
                else r = m;
            }
            else{
                if(nums[l] <= target && target <= nums[m]) r = m;
                else l = m;
            }
        }
        if(nums[l]==target) return l;
        if(nums[r]==target) return r;
        return -1;        
    }
};
