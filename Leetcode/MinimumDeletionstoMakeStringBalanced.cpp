// https://leetcode.com/problems/minimum-deletions-to-make-string-balanced

class Solution {
   public:
    int minimumDeletions(string s) {
        const auto an = count(s.cbegin(), s.cend(), 'a');
        const auto bn = count(s.cbegin(), s.cend(), 'b');

        size_t ans = min(an, bn);
        size_t a = 0;
        size_t b = 0;
        for (size_t i = 0; i < s.size(); ++i) {
            a += s[i] == 'a';
            b += s[i] == 'b';

            ans = min(ans, an - a + b);
        }

        return ans;
    }
};