// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one

class Solution {
   public:
    int numSteps(string s) {
        int steps = 0;

        while (s.length() != 1) {
            steps += 1;

            int last = s.back() - '0';

            s[s.length() - 2] += (last - (last & 1)) >> 1;
            last = last & 1;

            if (last) {
                s[s.length() - 1] += 1;
            } else {
                s.pop_back();
            }
        }

        return steps + (s[0] > '1');
    }
};