// https://leetcode.com/problems/longest-string-chain

class Solution {
   public:
    bool check(const string& from, const string& to) {
        for (int i = 0; i < to.length(); ++i) {
            if (from == to.substr(0, i) + to.substr(i + 1)) {
                return true;
            }
        }
        return false;
    }

    int longestStrChain(std::vector<std::string>& words) {
        int n = words.size();

        vector<vector<int>> grid(n);

        sort(words.begin(), words.end(), [](const string& a, const string& b) {
            return a.length() < b.length();
        });

        for (int i = 0; i < n; ++i) {
            for (int j = i + 1; j < n; ++j) {
                const string& from = words[i];
                const string& to = words[j];

                if (to.length() - from.length() > 1) {
                    break;
                }

                if (to.length() - from.length() == 1 && check(from, to)) {
                    grid[i].push_back(j);
                }
            }
        }

        vector<int> dp(n, 0);
        int ans = 0;

        for (int i = 0; i < n; ++i) {
            for (int to : grid[i]) {
                dp[to] = max(dp[to], dp[i] + 1);
                ans = max(ans, dp[to]);
            }
        }

        return ans + 1;
    }
};