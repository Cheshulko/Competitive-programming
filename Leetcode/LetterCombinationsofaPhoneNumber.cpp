class Solution {
public:
	
	vector<string> ans;
	void Go(string &ss, string& s, int x, char arr[9][4]){
		if (x == s.length() - 1){
			if (s[x] == '0' || s[x] == '1'){
				ans.push_back(ss);
			}
			else {
				for (int i = 0; i < 4; ++i){
					if (arr[s[x] - '0' - 1][i] != ' '){
						ss.push_back(arr[s[x] - '0' - 1][i]);
						ans.push_back(ss);
						ss.pop_back(); 
					}
				}
			}
		}
		else{
			if (s[x] == '0' || s[x] == '1'){
				Go(ss, s, x + 1, arr);
			}
			else{
				for (int i = 0; i < 4; ++i){
					if (arr[s[x] - '0' - 1][i] != ' '){
						ss.push_back(arr[s[x] - '0' - 1][i]);
						Go(ss, s, x + 1, arr);
						ss.pop_back();
					}
				}
			}

		}
	}

	vector<string> letterCombinations(string digits) {
	    if(digits.length() == 0) return ans;
		char arr[9][4] =
		{
			{ ' ', ' ', ' ', ' ' },
			{ 'a', 'b', 'c', ' ' },
			{ 'd', 'e', 'f', ' ' },
			{ 'g', 'h', 'i', ' ' },
			{ 'j', 'k', 'l', ' ' },
			{ 'm', 'n', 'o', ' ' },
			{ 'p', 'q', 'r', 's' },
			{ 't', 'u', 'v', ' ' },
			{ 'w', 'x', 'y', 'z' }
		};
		string ss;
		Go(ss, digits, 0, arr);
		return ans;
	}
};
