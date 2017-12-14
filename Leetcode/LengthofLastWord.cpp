class Solution {
public:
    int lengthOfLastWord(string s) {
        int l = s.length() - 1;
        int cnt = 0;
        while(l >= 0 && s[l] == ' ') --l;
        for(int i = l; i >= 0; --i){
            if(s[i] == ' ') return cnt;
            ++cnt;
        }
        return cnt;
    }
};
