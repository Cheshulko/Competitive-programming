class Solution {
public:
vector<string> ans;

map<int, vector<string> > have;

vector<string> generateParenthesis(int n) {
	if (have[n] != vector<string>()) return have[n];	
	vector<string> res;
	if (n == 0){
		res.push_back("");
		return res;
	}
	if (n == 1){
		res.push_back("()");
		return res;
	}
	for (int i = 0; i <= n - 1; ++i){
		vector<string> f = generateParenthesis(i), s = generateParenthesis(n - 1 - i);
		for (int k = 0; k < f.size(); ++k){			
			for (int l = 0; l < s.size(); ++l){
				res.push_back("(" + f[k] + ")" + s[l]);
			}
		}	
	}	
	return have[n] = res;
}
};
