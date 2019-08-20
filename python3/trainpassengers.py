c, n = map(int, input().split())
p = 0
f = False
for _ in range(n):
    l, e, s = map(int, input().split())
    p -= l
    if p < 0:
        f = True
        break
    p += e
    if p > c:
        f = True
        break
    if s > 0 and c - p > 0:
        f = True
        break
print('impossible' if f or p > 0 else 'possible')
