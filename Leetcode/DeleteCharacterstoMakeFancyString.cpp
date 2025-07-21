// https://leetcode.com/problems/delete-characters-to-make-fancy-string

class Solution {
   public:
    string makeFancyString(string s) {
        string ans;

        for (size_t i = 0; i < s.size(); ++i) {
            const auto sz = ans.size();
            if (sz > 1) {
                if (ans[sz - 2] == ans[sz - 1] && ans[sz - 1] == s[i]) {
                    // skip
                } else {
                    ans.push_back(s[i]);
                }
            } else {
                ans.push_back(s[i]);
            }
        }

        return ans;
    }
};