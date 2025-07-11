// https://leetcode.com/problems/meeting-rooms-iii

class Solution {
   public:
    int mostBooked(size_t n, vector<vector<int>>& meetings) {
        using i64 = long long;
        using Room = pair<i64, size_t>;
        using Meeting = pair<pair<int, int>, size_t>;

        vector<size_t> count(n, 0);

        multiset<Room> rooms;
        for (size_t i = 0; i < n; ++i) {
            rooms.insert({0, i});
        }

        set<Meeting> meetingsOrd;
        for (size_t i = 0; i < meetings.size(); ++i) {
            const auto& meeting = meetings[i];
            meetingsOrd.insert({{meeting[0], meeting[1]}, i});
        }

        i64 cur_time = 0;
        for (const auto& meeting : meetingsOrd) {
            cur_time = max(cur_time, (i64)meeting.first.first);
            {
                vector<Room> toUpdate;
                for (const auto& room : rooms) {
                    if (room.first <= cur_time) {
                        toUpdate.push_back(room);
                    }
                }

                for (const auto room : toUpdate) {
                    rooms.erase(room);
                    rooms.insert({cur_time, room.second});
                }
            }
            const auto room = *rooms.begin();
            rooms.erase(room);

            cur_time = max(cur_time, room.first);
            ++count[room.second];

            rooms.insert(
                {cur_time + (meeting.first.second - meeting.first.first),
                 room.second});
        }

        size_t best = 0;
        for (size_t i = 0; i < n; ++i) {
            if (count[i] > count[best]) {
                best = i;
            }
        }

        return (int)best;
    }
};