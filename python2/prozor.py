from __future__ import print_function
if __name__ == '__main__':
    r, s, k = map(int, raw_input().split())
    m = [raw_input() for i in range(r)]
    k -= 1
    fm = 0
    sx = 0
    sy = 0
    for i in range(r):
        for j in range(s):
            f = 0
            for l in range(1, k):
                if i + l + 1 >= r:
                    break
                for p in range(1, k):
                    if j + p + 1 >= s:
                        break
                    f += 1 if i + l < r and j + p < s and m[i + l][j + p] == "*" else 0
            if f > fm:
                sx = j
                sy = i
                fm = f

    print(fm)
    for i in range(r):
        for j in range(s):
            if (i == sy and j == sx) or (i == sy and j == sx + k) or (i == sy + k and j == sx) or (i == sy + k and j == sx + k):
                print("+", end="")
            elif (i == sy or i == sy + k) and sx < j < sx + k:
                print("-", end="")
            elif (j == sx or j == sx + k) and sy < i < sy + k:
                print("|", end="")
            else:
                print(m[i][j], end="")
        print("")

