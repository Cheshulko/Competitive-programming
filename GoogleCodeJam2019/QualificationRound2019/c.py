def gcd(a, b):
    while b:
        a, b = b, a % b
    return a


T = int(input())
for t in range(T):
    N, L = [int(x) for x in input().split()]
    arr = [int(x) for x in input().split()]

    ans = [None] * (L + 1)

    diff_ind = -1

    for i in range(L):
        if arr[i] != arr[i + 1]:
            diff_ind = i
            break

    brr = [(None, None)] * (L + 1)

    c = gcd(arr[diff_ind], arr[diff_ind + 1])
    brr[diff_ind + 1] = (c, diff_ind + 1)

    lc = c
    for i in range(diff_ind, -1, -1):
        lc = arr[i] // lc
        brr[i] = (lc, i)

    lc = c
    for i in range(diff_ind + 1, L):
        lc = arr[i] // lc
        brr[i + 1] = (lc, i + 1)

    se = sorted(set([x[0] for x in brr]))
    mp = dict()

    for i, x in enumerate(se):
        mp[x] = chr(65 + i)

    for i, x in enumerate(brr):
        ans[x[1]] = mp[x[0]]

    print("Case #{}: ".format(t + 1) + "".join(ans))
