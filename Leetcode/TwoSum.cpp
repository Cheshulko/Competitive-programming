class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        vector<int> res;
        map<int, int> have;
        for(int i = 0; i < nums.size(); ++i){
            if(have[target - nums[i]]){
                res.push_back(have[target - nums[i]]); res.push_back(i + 1);
                return res;
            }
            else{
                have[nums[i]] = i + 1;
            }
        }
    }
};
