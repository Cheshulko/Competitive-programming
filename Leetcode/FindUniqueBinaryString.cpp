// https://leetcode.com/problems/find-unique-binary-string

class Solution {
   public:
    string findDifferentBinaryString(vector<string>& nums) {
        sort(nums.begin(), nums.end());
        nums.push_back("");

        auto inc = [](string& ans) {
            const auto n = ans.length();

            int carry = 1;
            for (int i = n - 1; i >= 0; --i) {
                ans[i] += carry;
                carry = 0;
                if (ans[i] > '1') {
                    ans[i] = '0';
                    carry = 1;
                }
            }
        };

        string cur(nums[0].size(), '0');
        for (size_t i = 0; i < nums.size();) {
            if (nums[i] == cur) {
                inc(cur);
                ++i;
            } else {
                return cur;
            }
        }

        unreachable();
    }
};