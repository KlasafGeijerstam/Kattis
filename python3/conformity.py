c = {}
for t in [tuple(sorted(map(int, input().split()))) for _ in range(int(input()))]:
    if t not in c:
        c[t] = 0
    c[t] += 1
m = max(c.values())
print(sum([x for x in c.values() if x == m]))
