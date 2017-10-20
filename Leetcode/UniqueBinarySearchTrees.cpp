class Solution {
public:
    int numTrees(int n) {
       long long dp[n + 1];
       dp[0] = 1; 
       for(int i = 1; i <= n; ++i) dp[i] = (2 * (2*(i-1) + 1) * dp[i-1]) / (i + 1);
       return dp[n];      
    }
};
