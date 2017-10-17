class Solution {
public:
    string getPermutation(int n, int k) {
	int f[10] = { 1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880 };
	vector<int> arr;
	string ans;
	for (int i = 1; i <= n; ++i) arr.push_back(i);
	for (int i = n; i >= 1; --i){
		int x = ((k - 1) / f[i - 1]);
		k -= x * f[i - 1];
		ans.push_back(arr[x] + '0');
		arr.erase(arr.begin() + x);
	}
	return ans;
    }
};
