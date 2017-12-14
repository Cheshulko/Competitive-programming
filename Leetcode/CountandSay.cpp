class Solution {
public:
    string countAndSay(int n) {
        string ans = "1";
        string N = "1";
        
        for(int i = 1; i < n; ++i){
            ans = "";
            for(int j = N.size() - 1; j >= 0; --j){
                char x = N[j];
                int cnt = 0;
                while(j >= 0 && N[j] == x){
                    ++cnt;
                    --j;
                }
                ++j;
                ans.push_back(x); ans.push_back(char(cnt + '0'));
            }
            reverse(ans.begin(), ans.end());
            N = ans;
        }
        return ans;
    }
};
