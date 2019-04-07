x = [[1,  2,  3,  4,  5],
     [6,  7,  8,  9,  10],
     [11, 12, 13, 14, 15],
     [16, 17, 18, 19, 20]]

# x = [[1,  2,  3,  4,  5],
#      [6,  7,  8,  9,  10],
#      [11, 12, 13, 14, 15],
#      [16, 17, 18, 19, 20],
#      [1,  2,  3,  4,  5],
#      [6,  7,  8,  9,  10],
#      [11, 12, 13, 14, 15],
#      [16, 17, 18, 19, 20]]

# x = [[1,  2,  3,  4,  5],
#      [6,  7,  8,  9,  10],
#      [11, 12, 13, 14, 15]]

# x = [[1,  2,  3,  4,  5]]

# x = [[1],
#      [6],
#      [11],
#      [16],
#      [1],
#      [6],
#      [11],
#      [16]]

n = len(x)
m = len(x[0])

dn = [0, 1, 0, -1]
dm = [1, 0, -1, 0]

d = [m + 1, n + 1, m + 1, n + 1]
i, j = -1, -1

mn = min(n, m)

for cnt in range(mn // 2):
    i, j = cnt, cnt
    d = [x - 2 for x in d]

    for k in range(4):
        for p in range(d[k]):
            print(x[i][j], end='\n')
            i += dn[k]
            j += dm[k]

if mn % 2:
    i += 1
    j += 1
    if mn == n:
        for p in range(d[0] - 1):
            print(x[i][j], end='\n')
            i += dn[0]
            j += dm[0]
    else:
        for p in range(d[1] - 1):
            print(x[i][j], end='\n')
            i += dn[1]
            j += dm[1]


# Given a N by M matrix of numbers, print out the matrix in a clockwise spiral.

# For example, given the following matrix:

# [[1,  2,  3,  4,  5],
#  [6,  7,  8,  9,  10],
#  [11, 12, 13, 14, 15],
#  [16, 17, 18, 19, 20]]
# You should print out the following:

# 1
# 2
# 3
# 4
# 5
# 10
# 15
# 20
# 19
# 18
# 17
# 16
# 11
# 6
# 7
# 8
# 9
# 14
# 13
# 12
