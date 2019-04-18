class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        if(nums.empty()) return 0;
        int ans = 0;
        int cur = 0;
        for(size_t i = 0; i < nums.size(); ++i){
            cur = (cur + 1)*nums[i];
            ans = max(ans, cur); 
        }
        return ans;
    }
};