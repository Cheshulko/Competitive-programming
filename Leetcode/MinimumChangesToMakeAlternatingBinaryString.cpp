// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string

class Solution {
   public:
    int minOperations(string s) {
        int case1 = 0, case2 = 0;
        for (size_t i = 0; i < s.length(); ++i) {
            case1 += (1 - (i & 1)) & (s[i] == '0');
            case1 += (i & 1) & (s[i] == '1');

            case2 += (1 - (i & 1)) & (s[i] == '1');
            case2 += (i & 1) & (s[i] == '0');
        }

        return min(case1, case2);
    }
};