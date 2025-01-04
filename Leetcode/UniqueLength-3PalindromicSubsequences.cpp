// https://leetcode.com/problems/unique-length-3-palindromic-subsequences

class Solution {
   public:
    int countPalindromicSubsequence(string s) {
        array<vector<int>, 28> chars;
        for (int i = 0; i < s.length(); ++i) {
            chars[s[i] - 'a'].push_back(i);
        }

        int ans = 0;
        for (int i = 0; i < 28; ++i) {
            set<int> middle;

            if (chars[i].size() > 1) {
                int start = *chars[i].begin();
                int end = *(--chars[i].end());

                for (int k = 0; k < 28; ++k) {
                    auto it =
                        upper_bound(chars[k].begin(), chars[k].end(), start);
                    if (it != chars[k].end() && *it < end) {
                        middle.insert(*it);
                    }
                }
            }

            ans += middle.size();
        }

        return ans;
    }
};