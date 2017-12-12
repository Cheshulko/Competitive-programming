class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>>ans;
        if(nums.size() == 0) return ans;
        ans.push_back(nums);
        while(1){
            bool done = 0;
            for(int j = nums.size() - 2; j >= 0 && !done; --j){
                if(nums[j + 1] > nums[j]){
                    for(int m = nums.size() - 1; m > j && !done; --m){
                        if(nums[m] > nums[j]){
                            swap(nums[m], nums[j]);
                            sort(nums.begin() + j + 1, nums.end());
                            ans.push_back(nums);
                            done = 1;
                        }
                    }
                }
            }
            if(!done) break;
        }
        return ans;
    }
};
