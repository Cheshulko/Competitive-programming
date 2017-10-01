class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        vector<vector<string>> ans;
        map<string, int> m;
        int now = 1;
        for(int i = 0; i < strs.size(); ++i){
            string t = strs[i];
            sort(t.begin(), t.end());
            int x = m[t];
            if(x != 0){
                ans[x - 1].push_back(strs[i]);        
            }
            else{
                m[t] = now;
                ans.push_back(vector<string>());
                ans[now - 1].push_back(strs[i]);
                ++now;
            }           
        }
        return ans;
    }
};