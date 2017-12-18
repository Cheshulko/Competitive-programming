class Solution {
public:
    bool isValid(string s) {
        vector<int> lastOpened;
        int F = 0, S = 0, T = 0;
        int len = s.length();
        for(int i = 0; i < len; ++i){
            if(s[i] == '('){
                F++;
                lastOpened.push_back(1);
            }
            if(s[i] == '{'){
                S++;
                lastOpened.push_back(2);
            }
            if(s[i] == '['){
                T++;
                lastOpened.push_back(3);
            }
            if(s[i] == ')'){
                if(F == 0 || lastOpened[F + S + T - 1] != 1) return false;
                F--;
                lastOpened.pop_back();
            }
            if(s[i] == '}'){
                if(S == 0 || lastOpened[F + S + T - 1] != 2) return false;
                S--;
                lastOpened.pop_back();
            }
            if(s[i] == ']'){
                if(T == 0 || lastOpened[F + S + T - 1] != 3) return false;
                T--;
                lastOpened.pop_back();
            }
        }
        if(F != 0 || S != 0 || T != 0) return false;
        return true;
    }
};
