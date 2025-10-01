// https://leetcode.com/problems/water-bottles

int numWaterBottles(int numBottles, int numExchange) {
    int ans = 0;
    int empty = 0;

    while (numBottles) {
        ans += numBottles;
        int all = empty + numBottles;
        numBottles = all / numExchange;
        empty = all % numExchange;
    }

    return ans;
}