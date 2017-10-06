class Solution {
public:

    bool dfs(vector<vector<char>>& b, vector<vector<bool>>& used, string &w, int num, int i, int j, int n, int m){
        bool Ok = false;
        if(num == w.size()) return 1;
        if(!Ok && i - 1 >= 0 && !used[i - 1][j] && b[i - 1][j] == w[num]){
            used[i - 1][j] = 1;
            Ok |= dfs(b, used, w, num + 1, i - 1, j, n, m);
            used[i - 1][j] = 0;
        }
        if(!Ok && i + 1 < n && !used[i + 1][j] && b[i + 1][j] == w[num]){
            used[i + 1][j] = 1;
            Ok |= dfs(b, used, w, num + 1, i + 1, j, n, m);
            used[i + 1][j] = 0;
        }
        if(!Ok && j - 1 >= 0 && !used[i][j - 1] && b[i][j - 1] == w[num]){
            used[i][j - 1] = 1;
            Ok |= dfs(b, used, w, num + 1, i, j - 1, n, m);
            used[i][j - 1] = 0;
        }
        if(!Ok && j + 1 < m && !used[i][j + 1] && b[i][j + 1] == w[num]){
            used[i][j + 1] = 1;
            Ok |= dfs(b, used, w, num + 1, i, j + 1, n, m);
            used[i][j + 1] = 0;
        }
        return Ok;
    }

    bool exist(vector<vector<char>>& b, string w) {
        if(w.size() == 0) return 1;
        int n = b.size();
        if(n == 0) return 0;
        int m = b[0].size();
        vector<vector<bool>> used;
        used.resize(n + 1);
        for(int i = 0; i < n; ++i)used[i].assign(m + 1, 0);
        bool Ok = false;
        for(int i = 0; i < n && !Ok; ++i){
            for(int j = 0; j < m && !Ok; ++j){
                if(b[i][j] == w[0]) {
                    used[i][j] = 1;
                    Ok |= dfs(b, used, w, 1, i, j, n, m);
                    used[i][j] = 0;
                }
            }
        }
        return Ok;
    }
};
