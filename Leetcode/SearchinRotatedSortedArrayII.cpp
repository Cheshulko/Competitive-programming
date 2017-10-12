class Solution {
public:
bool check(vector<int>& nums, int l, int r){
    for(int i = l; i < r; ++i) if(nums[i] > nums[i + 1]) return false;
    return true;
}

    bool search(vector<int>& nums, int target) {
        int l = 0, r = nums.size() - 1;
        while(r - l > 1){
            int m = (l + r) / 2;
            if(check(nums, m, r)){
                if(nums[m] <= target && target <= nums[r]) l = m;
                else r = m;
            }
            else{
                if(nums[l] <= target && target <= nums[m]) r = m;
                else l = m;
            }
        }
        return (nums[l] == target || nums[r] == target);
    }
};
