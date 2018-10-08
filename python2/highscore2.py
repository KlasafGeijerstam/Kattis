def score(a,b,c):
    return a*a + b*b + c*c + 7*min(a,b,c)

n = input()
for i in range(n):
    m = map(int,raw_input().split())
    d = int(m[3])
    m = m[:-1]
    m.sort()
    ans = 0
    for i in range(min(5, d)+1):
        cp = list(m)
        for j in range(i):
            cp[0] += 1
            cp.sort()
        cp[2] += d - i
        ans = max(ans, score(cp[0], cp[1], cp[2]))
print(ans) 
