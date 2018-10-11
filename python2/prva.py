r, c = map(int, raw_input().split())
m = [raw_input() for _ in range(r)]
b = "z"
for i in range(r):
    for j in range(c):
        if i == 0 or m[i-1][j] == "#":
            l = ""
            for k in range(i, r):
                if m[k][j] == "#":
                    break
                l += m[k][j]
            if len(l) > 1 and l < b:
                b = l
        if j == 0 or m[i][j-1] == "#":
            l = ""
            for k in range(j, c):
                if m[i][k] == "#":
                    break
                l += m[i][k]
            if len(l) > 1 and l < b:
                b = l
print b
