class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
	int n = matrix.size();
	for (int i = 0; i < n / 2; ++i){
		for (int j = i; j < n - i - 1; ++j){
			int t;
			t = matrix[j][n - i - 1];
			matrix[j][n - i - 1] = matrix[i][j];
			swap(t, matrix[n - i - 1][n - j - 1]);
			swap(t, matrix[n - j - 1][i]);
			swap(t, matrix[i][j]);
		}
	}
    }
};
