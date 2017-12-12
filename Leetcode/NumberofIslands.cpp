class Solution {
public:

vector< vector<int> > used;

void dfs(int i, int j, int n, int m, vector<vector<char>>& board){
	used[i][j] = 1;
	
	if (i - 1 >= 0){
		if (!used[i - 1][j] && board[i - 1][j] == '1'){
			dfs(i - 1, j, n, m, board);
		}		
	}
	if (j - 1 >= 0){
		if (!used[i][j - 1] && board[i][j - 1] == '1'){
			dfs(i, j - 1, n, m, board);
		}
	}

	if (i + 1 < n){
		if (!used[i + 1][j] && board[i + 1][j] == '1'){
			dfs(i + 1, j, n, m, board);
		}
	}

	if (j + 1 < m){
		if (!used[i][j + 1] && board[i][j + 1] == '1'){
			dfs(i, j + 1, n, m, board);
		}
	}
}

int numIslands(vector<vector<char>>& board) {
        int n = board.size();
	    if (n == 0) return 0;
	    int m = board[0].size();
	    vector<int> t; t.assign(m + 1, 0);
	    used.assign(n + 1, t);
	    int ans = 0;
	    for (int i = 0; i < n; ++i){
		    for (int j = 0; j < m; ++j){
			    if (!used[i][j] && board[i][j] == '1'){
			    	ans++;
			    	dfs(i, j, n, m, board);
			    }
		    }
	    }
	    return ans;
    }
};
