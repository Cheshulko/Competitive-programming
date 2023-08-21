
// https://leetcode.com/problems/repeated-substring-pattern

#include <cmath>
#include <iostream>
#include <set>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
   public:
    bool repeatedSubstringPattern(string s) {
        int j, len = s.length();
        vector<int> p(len, 0);
        for (int i = 1; i < len; ++i) {
            j = p[i - 1];
            while ((j > 0) && (s[i] != s[j])) {
                j = p[j - 1];
            }
            if (s[i] == s[j]) {
                j++;
            }
            p[i] = j;
        }
        int x = len - p[len - 1] - 1;
        return x != len - 1 && s.substr(0, x + 1) == s.substr(len - x - 1, len);
    }
};