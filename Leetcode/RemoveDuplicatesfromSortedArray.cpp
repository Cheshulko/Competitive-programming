class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        if(nums.size() == 0) return 0;
        int n = 0, len = nums.size();
        for(int i = 0; i < len; ++i){
            nums[n] = nums[i];
            while(i < len && nums[n] == nums[i]) ++i;
            ++n;
            --i;
        }
        return n;
    }
};
