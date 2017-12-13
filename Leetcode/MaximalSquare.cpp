class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        int n = matrix.size();
        if(n == 0) return 0;
        int m = matrix[0].size();
        if(m == 0) return 0;
        int **dp = new int*[n + 1];
        for(int i = 0; i < n + 1; ++i) dp[i] = new int[m + 1];
        for(int i = 0; i < n; ++i) dp[i][0] = (matrix[i][0] == '1');
        for(int j = 0; j < m; ++j) dp[0][j] = (matrix[0][j] == '1');
        for(int i = 1; i < n; ++i){
            for(int j = 1; j < m; ++j){
                dp[i][j] = (matrix[i][j] == '1' ? min(min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]) + 1 : 0);
            }
        }
        int mx = 0;
        for(int i = 0; i < n; ++i) for(int j = 0; j < m; ++j) mx = max(mx, dp[i][j]);
        return mx*mx;
    }
};
