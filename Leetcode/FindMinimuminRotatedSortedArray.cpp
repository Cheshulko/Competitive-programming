class Solution {
public:
	int findMin(vector<int>& nums) {
		int ans = nums[0];
		int l = 0, r = nums.size();
		while (r - l > 1){
			int m = (r + l) / 2;
			if (nums[l] < nums[m]){
				ans = min(ans, nums[l]);
				l = m;
			}
			else{
				ans = min(ans, nums[m]);
				r = m;
			}
		}
		return ans;
	}
};
