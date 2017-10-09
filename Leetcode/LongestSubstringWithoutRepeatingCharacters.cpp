class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int i = 0, j = 0, n = s.length();
        int used[256];
        memset(used, 0, sizeof used);
        int ans = 0;
        int len = 0;
        while(j < n){
            while(j < n && !used[s[j]]){
                ++len, used[s[j]] = 1, ++j;
            }
            ans = max(ans, len);
            
           
		if (j < n && used[s[j]]){
			while (s[i] != s[j]){
				used[s[i]] = 0;
				++i;
				--len;
			}
			used[s[i]] = 0;
			++i, --len;
		}
        }
        return ans;
    }
};
