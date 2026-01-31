// https://leetcode.com/problems/find-smallest-letter-greater-than-target

char nextGreatestLetter(char* letters, int lettersSize, char target) {
    int l = -1;
    int r = lettersSize;

    while (r - l > 1) {
        int m = (r + l) >> 1;

        if (letters[m] <= target) {
            l = m;
        } else {
            r = m;
        }
    }

    return letters[r % lettersSize];
}