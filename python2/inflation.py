x = int(raw_input())
c = map(float, raw_input().split())
c.sort()
p = zip(c, range(1, x+1))
m = 100
for k in p:
    if k[0] > k[1]:
        print "impossible"
        exit(0)
    m = min(m, k[0]/k[1])
print m
