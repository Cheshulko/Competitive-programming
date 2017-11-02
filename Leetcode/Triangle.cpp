Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.

For example, given the following triangle

[
     [2],
    [3,4],
   [6,5,7],
  [4,1,8,3]
]

The minimum path sum from top to bottom is 11 (i.e., 2 + 3 + 5 + 1 = 11). 

class Solution {
public:
    int minimumTotal(vector<vector<int>>& tr) {
        int cnt = tr.size();
        if(cnt == 0) return 0;
        int dp[2][tr.size()];
        int type = 0;
        dp[type][0] = tr[0][0];
        for(int i = 0; i < cnt - 1; ++i){
            
            for(int j = 0; j < tr[i].size() + 1; ++j) dp[type^1][j] = 999999999;
            for(int j = 0; j < tr[i].size(); ++j){
                dp[type^1][j] =     min(dp[type^1][j],     dp[type][j] + tr[i + 1][j]    );
                dp[type^1][j + 1] = min(dp[type^1][j + 1], dp[type][j] + tr[i + 1][j + 1]);
            }
            
            type ^= 1;
        }
        int ans = 9999999999;
        for(int i = 0; i < tr[cnt - 1].size(); ++i)ans = min(ans, dp[type][i]);
        return ans;
     }
};
