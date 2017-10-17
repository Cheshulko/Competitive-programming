class Solution {
public:
    void setZeroes(vector<vector<int>>& matrix) {
        int n = matrix.size();
        if(n == 0) return;
        int m = matrix[0].size();
        int r[n + 1], c[m + 1];
        for(int i = 0; i <= n; ++i) r[i] = 0;
        for(int j = 0; j <= m; ++j) c[j] = 0;
        for(int i = 0; i < n; ++i){
            for(int j = 0; j < m; ++j) {
                if(matrix[i][j] == 0) r[i] = 1, c[j] = 1;
            }
        }
        for(int i = 0; i < n; ++i){
            if(r[i]==1){
                for(int j = 0; j < m; ++j){
                    matrix[i][j] = 0;
                }
            }    
            else{
                for(int j = 0; j < m; ++j){
                    if(c[j]==1) matrix[i][j] = 0;
                }
            }
        }
    }
};
