#include <stdio.h>
#include <vector>

using std::vector;

int N, M;

int main()
{
    scanf("%d %d", &N, &M);
    vector<vector<int>> v(N + 1, vector<int>(M + 1));
    v[0][0] = 1;
    for (int i = 0; i < N; ++i)
    {
        for (int j = 0; j < M; ++j)
        {
            v[i + 1][j] += v[i][j];
            v[i][j + 1] += v[i][j];
        }
    }
    printf("%d\n", v[N - 1][M - 1]);

    return 0;
}