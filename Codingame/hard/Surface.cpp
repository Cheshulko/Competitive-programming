// https://www.codingame.com/ide/puzzle/surface

#include <algorithm>
#include <iostream>
#include <queue>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

/*

6
6
######
#OO#O#
#OO#O#
#O####
##OOO#
######
1
1 1
3
4 1
1 1
4 4

1 1
O
1
0 0

5 5
OOOOO
OOOOO
OOOOO
OOOOO
OOOOO
1
1 1

4
4
####
##O#
#OO#
####
1
1 2
3
0 0
1 2
2 1
*/

struct Point {
    int x;
    int y;

    Point() : x(0), y(0){};
    Point(const int& x, const int& y) : x(x), y(y){};
    Point(const Point& other) : x(other.x), y(other.y){};

    Point operator+(const Point& other) const {
        return Point{x + other.x, y + other.y};
    };

    Point& operator=(const Point& other) {
        x = other.x;
        y = other.y;

        return *this;
    };

    bool operator==(const Point& other) const {
        return x == other.x && y == other.y;
    };

    bool operator<(const Point& other) {
        return x < other.x || (x == other.x && y < other.y);
    };

    size_t operator()(const Point& point) const noexcept {
        size_t hash_x = hash<int>{}(point.x);
        size_t hash_y = hash<int>{}(point.y);

        return hash_x ^ hash_y;
    };
};

std::ostream& operator<<(std::ostream& os, const Point& point) {
    return os << "Point { x = " << point.x << ", y = " << point.y << " }";
}

namespace std {
template <>
struct hash<Point> {
    std::size_t operator()(const Point& point) const noexcept {
        return point(point);
    }
};
}  // namespace std

class Grid {
    vector<string> _grid;

    // memory bullshit using
    vector<vector<int>> _v;
    vector<vector<int>> _a_x;
    vector<vector<int>> _a_y;

    bool _isValid(const Point& coor) const { return at(coor.x, coor.y) == 'O'; }

    bool _canGo(const Point& coor) const {
        return coor.x >= 0 && coor.x < l() && coor.y >= 0 && coor.y < h();
    }

    void _bfs(const Point& activ, const Point& coor, int& sum) {
        queue<Point> q;
        q.push(coor);

        _a_x[coor.x][coor.y] = activ.x + 1;
        _a_y[coor.x][coor.y] = activ.y + 1;

        while (!q.empty()) {
            auto cur = q.front();
            q.pop();
            ++sum;

            for (int i = 0; i < 4; ++i) {
                const auto& dir = Grid::dirs[i];
                const auto to = cur + dir;

                if (_canGo(to) && _a_x[to.x][to.y] == 0 && _isValid(to)) {
                    _a_x[to.x][to.y] = activ.x + 1;
                    _a_y[to.x][to.y] = activ.y + 1;

                    q.push(to);
                }
            }
        }
    }

   public:
    static const Point dirs[4];

    int l() const { return _grid[0].length(); }

    int h() const { return _grid.size(); }

    char at(const int& x, const int& y) const { return _grid[y][x]; }

    int ans(const Point& coor) {
        if (_isValid(coor)) {
            if (_a_x[coor.x][coor.y] > 0) {
                auto x = _a_x[coor.x][coor.y] - 1;
                auto y = _a_y[coor.x][coor.y] - 1;
                return _v[x][y];
            }

            int sum = 0;
            _bfs(coor, coor, sum);
            _v[coor.x][coor.y] = sum;
            return sum;
        } else {
            return 0;
        }
    }

    void input() {
        int l, h;
        cin >> l >> h;
        cin.ignore();
        _grid.resize(h);
        for (int i = 0; i < h; i++) {
            getline(cin, _grid[i]);
        }

        _a_x.resize(l);
        _a_y.resize(l);
        _v.resize(l);
        for (int i = 0; i < l; ++i) {
            _a_x[i].resize(h);
            _a_y[i].resize(h);
            _v[i].resize(h);
        }
    };
};

const Point Grid::dirs[4] = {
    Point{-1, 0},
    Point{1, 0},
    Point{0, 1},
    Point{0, -1},
};

int main() {
    Grid grid;
    grid.input();

    int n;
    cin >> n;
    vector<int> result;
    result.resize(n);
    for (int i = 0; i < n; i++) {
        int x, y;
        cin >> x >> y;
        result[i] = grid.ans({x, y});
    }

    for (int i = 0; i < n; i++) {
        cout << result[i] << endl;
    }
}
// g++ test.cpp -std=c++17 -o test && ./test