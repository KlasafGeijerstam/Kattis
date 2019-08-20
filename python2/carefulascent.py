x, y = map(int, raw_input().split())
n = int(raw_input())
s = []
for _ in range(n):
    a,b,c = raw_input().split()
    s.append((int(a), int(b), float(c)))
s.sort(key=lambda c: c[0])

ps = 0
k = 0
for h in s:
    k += h[0] - ps
    k += (h[1] - h[0]) * h[2]
    ps = h[1]
k += (y - ps)

print "%.8f" % (x/float(k))
