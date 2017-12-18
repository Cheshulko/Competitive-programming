class Solution {
public:
    int myAtoi(string str) {
        auto first = str.begin(), last = str.end();
        while (isspace(*first) && first != last) { first++; }

        int sgn = 1;
        if (*first == '-' || *first == '+') {
            sgn = *first == '-' ? -1 : 1;
            first++;
        }

        unsigned long long int result = 0;
        for ( ; isdigit(*first) && first != last; ++first) {
            result = 10 * result + *first - '0';
            if (result >= numeric_limits<int>::max() && sgn == 1) { return numeric_limits<int>::max(); }
            if (result > numeric_limits<int>::max() && sgn == -1) { return numeric_limits<int>::min(); }
        }
        return sgn*result;
    }
};
