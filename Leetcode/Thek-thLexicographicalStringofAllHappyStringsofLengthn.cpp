// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n

class Solution {
   public:
    string getHappyString(int n, int k) {
        auto check = [](const string& s) -> bool {
            for (size_t i = 0; i < s.size() - 1; ++i) {
                if (s[i] == s[i + 1]) {
                    return false;
                }
            }

            return true;
        };

        auto inc = [](string& s) -> bool {
            for (int i = s.length() - 1; i >= 0; --i) {
                if (s[i] < 'c') {
                    s[i] += 1;

                    return true;
                }
                s[i] = 'a';
            }

            return false;
        };

        string ans(n, 'a');
        ans[n - 1] = 'a' - 1;
        for (; k > 0; --k) {
            if (!inc(ans)) {
                return "";
            }
            while (!check(ans)) {
                if (!inc(ans)) {
                    return "";
                }
            }
        }

        return ans;
    }
};