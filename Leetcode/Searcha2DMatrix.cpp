class Solution {
public:
    bool searchMatrix(vector<vector<int>>&mtx, int t) {
        int n = mtx.size();
        if(n ==0) return 0;
        int m = mtx[0].size();
        int l = 0, r = n - 1;
        while(r - l > 1){
            int m = (l + r) / 2;
            if(mtx[m][0] <= t) l = m;
            else r = m;
        }
        int row;
        if(mtx[r][0] <= t) row = r;
        else row = l;
        
        l = 0, r = m - 1;
        while(r - l > 1){
            int m = (l + r) / 2;
            if(mtx[row][m] <= t) l = m;
            else r = m;
        }
        return mtx[row][l] == t || mtx[row][r] == t;
        
    }
};
