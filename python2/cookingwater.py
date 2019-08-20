mi = 0
ma = 10000000
for _ in range(int(raw_input())):
    s,e = map(int, raw_input().split())
    ma = min(ma, e)
    mi = max(mi, s)

print "gunilla has a point" if ma >= mi else "edward is right"
