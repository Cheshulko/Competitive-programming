class Solution {
public:
    bool isPalindrome(string s) {
        int len = s.length();
        int j = len - 1;
        int i = 0;
        
        while(i < j){
            while( i < len && !(('0' <= s[i] && s[i] <= '9') || ('a' <= s[i] && s[i] <= 'z') || ('A' <= s[i] && s[i] <= 'Z')) ) ++i;
            while( j >= 0  && !(('0' <= s[j] && s[j] <= '9') || ('a' <= s[j] && s[j] <= 'z') || ('A' <= s[j] && s[j] <= 'Z')) ) --j;
            if(i < j) {
                if(tolower(s[i]) != tolower(s[j])) return false;
            }
            ++i, --j;
        }
        return true;
    }
};
