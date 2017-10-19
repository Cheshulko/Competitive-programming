class Solution {
public:
    int minSubArrayLen(int s, vector<int>& nums) {
        int n = nums.size();
        if(n == 0) return 0;
        int i = 0, j = 0, ans = 13123123, sum = 0;
        while(j < n){
            while(j < n && sum < s) sum += nums[j++];
            while(i < j && sum >= s){
                ans = min(ans, j - i);
                sum -= nums[i];
                ++i;
            }
        }
        return (ans == 13123123 ? 0 : ans);
    }
};
