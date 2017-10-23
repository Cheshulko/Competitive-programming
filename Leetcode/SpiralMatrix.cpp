class Solution {
public:
   vector<int> spiralOrder(vector<vector<int>>& matrix) {
	int n = matrix.size();
	vector<int> ans;
	int magic = 12312312;
	if (n == 0) return ans;
	int m = matrix[0].size();
	int cnt = 0;
	int i = 0, j = 0;
	while (cnt < n*m){
		bool  up = false;
		while (j < m && matrix[i][j] != magic){
			ans.push_back(matrix[i][j]);
			matrix[i][j] = magic;
			++j;
			++cnt;
			up = true;
			
		}
		if(up)--j;
		if(i + 1 < n) ++i;
		up = false;

		while (i < n && matrix[i][j] != magic){
			ans.push_back(matrix[i][j]);
			matrix[i][j] = magic;
			++i;
			++cnt;
			up = true;
		}
		if(up)--i;
		if(j - 1 >= 0)--j;
		up = false;

		while (j >=0 && matrix[i][j] != magic){
			ans.push_back(matrix[i][j]);
			matrix[i][j] = magic;
			--j;
			++cnt;
			up = true;
		}

		if(up)++j;
		if(i - 1 >= 0)--i;
		up = false;

		while (i>=0 && matrix[i][j] != magic){
			ans.push_back(matrix[i][j]);
			matrix[i][j] = magic;
			--i;
			++cnt;
			up = true;
		}
		if(up)++i;
		if(j + 1 < m)++j;
	}
	return ans;
}
};
