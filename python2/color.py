s,c,k = map(int, raw_input().split())
l = map(int, raw_input().split())
l.sort()
m = 0
ls = 0
mc = 1000000000000
for sock in l:
    if abs(sock - ls) > k or mc >= c:
        m += 1
        mc = 0
    ls = sock
    mc += 1
print m