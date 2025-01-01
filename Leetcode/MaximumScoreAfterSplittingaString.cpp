// https://leetcode.com/problems/maximum-score-after-splitting-a-string

class Solution {
   public:
    int maxScore(string s) {
        int ones = 0;
        for (char c : s) {
            if (c == '1') {
                ones += 1;
            }
        }

        int zeros = s[0] == '0';
        ones -= s[0] == '1';
        int ans = zeros + ones;
        for (int i = 1; i < s.size() - 1; ++i) {
            if (s[i] == '0') {
                zeros += 1;
            } else {
                ones -= 1;
            }
            ans = max(ans, zeros + ones);
        }

        return ans;
    }
};