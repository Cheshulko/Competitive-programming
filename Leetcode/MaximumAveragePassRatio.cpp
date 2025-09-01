// https://leetcode.com/problems/maximum-average-pass-ratio

class Solution {
   public:
    double maxAverageRatio(vector<vector<int>>& classes, int extraStudents) {
        const auto getDelta = [&](const auto& cl, const auto& ind) {
            const long long pass = cl[0];
            const long long total = cl[1];
            const auto num = ((pass + 1) * total - pass * (total + 1));
            const auto den = (total + 1) * total;

            return pair<double, size_t>{(1. * num / den), ind};
        };

        auto ind = 0;
        vector<pair<double, size_t>> data;
        data.reserve(classes.size());

        transform(classes.cbegin(), classes.cend(), std::back_inserter(data),
                  [&](const auto& cl) { return getDelta(cl, ind++); });

        priority_queue<pair<double, size_t>, vector<pair<double, size_t>>,
                       std::less<>>
            minHeap(less<>(), move(data));

        while (extraStudents-- > 0) {
            const auto [_, ind] = minHeap.top();
            minHeap.pop();

            ++classes[ind][0];
            ++classes[ind][1];

            minHeap.push(getDelta(classes[ind], ind));
        }

        double avg = 0;
        while (!minHeap.empty()) {
            const auto [_, ind] = minHeap.top();
            minHeap.pop();
            avg += 1. * classes[ind][0] / classes[ind][1];
        }

        return avg / classes.size();
    }
};