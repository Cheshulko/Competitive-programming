class Solution {
public:
    static bool cmp(string &s1, string &s2){
	return s1 + s2 > s2 + s1;
}

string largestNumber(vector<int>& nums) {
	vector<string> s;

	for (int i = 0; i < nums.size(); ++i){
		stringstream ss;
		ss << nums[i];
		s.push_back(string(ss.str()));
	}
	sort(s.begin(), s.end(), cmp);
	string ans = "";
	for (int i = 0; i < s.size(); ++i) ans += s[i];
	string res = "";
	for(int i = 0 ; i < ans.size(); ++i){
	    if(ans[i] != '0'){
	        res.insert(0, ans, i, ans.size() - i);
	        break;
	    } 
	}
	return (res.empty() ? "0" : ans);
    }
};
