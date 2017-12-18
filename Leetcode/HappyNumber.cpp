class Solution {
public:
map<long long, bool> used;
bool isHappy(long long n) {
	if (used[n] || n == 0) {
		return false;
	}
	if (n == 1) return true;
	long long ne = 0;
	used[n] = 1;
	while (n){
		int x = n % 10;
		x *= x;
		n /= 10;
		ne += x;
	}
	return isHappy(ne);
}
};
