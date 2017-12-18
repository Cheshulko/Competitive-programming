class Solution {
public:
    int trailingZeroes(int n) {
        int cnt5 = 0, cnt2 = 0;
        long long five = 5;
        long long two = 2;
        while(1){
            int up = n / five;
            if(up == 0) break;
            cnt5 += up;
            five *= 5;
        }
         while(1){
            int up = n / two;
            if(up == 0) break;
            cnt2 += up;
            two *= 2;
        }
        
        return min(cnt5, cnt2);
    }
};
