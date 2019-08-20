n, y = map(int, raw_input().split())
x = set(range(n))
for _ in range(y):
    p = int(raw_input())
    if p in x:
        x.remove(p)
for j in range(n):
    if j in x:
        print j

print "Mario got %d of the dangerous obstacles." % (n - len(x))