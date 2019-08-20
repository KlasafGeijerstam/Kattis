n, q = map(int, raw_input().split())
l = [int(i) for i in raw_input().split()]
l = [0] + l
for _ in range(q):
    x, y, z = map(int, raw_input().split())
    if x == 1:
        l[y] = z
    else:
        print abs(l[y] - l[z])
