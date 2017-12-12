class Solution {
public:
    int rangeBitwiseAnd(int m, int n) {
	int cur = 0;
	int ans = 0;
	int mask = (1ll << 31) - 1;
	
	while (m >= (1ll << cur)){
 		if ((m & (1ll << cur)) != 0){
			int y = m ^ mask;
			y = ((y >> cur) << cur);
			y = y ^ mask;
			if (n <= y) ans |= (1 << cur);
		}
		++cur;
	}
	return ans;
}
};
