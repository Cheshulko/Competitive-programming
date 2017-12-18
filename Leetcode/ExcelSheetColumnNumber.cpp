class Solution {
public:
    int titleToNumber(string s) {
        int ans = 0;
        int len = s.length();
        long long p = 1;
        for(int i = len - 1; i >= 0; --i){
            ans += p * (s[i] - 'A' + 1);
            p *= 26;
        }
        return ans;
    }
};
