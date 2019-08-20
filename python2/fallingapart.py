raw_input()
p = map(int, raw_input().split())
p.sort()
t = 0
b = 0
a = 0
while p:
    if t % 2 == 0:
        a += p[-1]
    else:
        b += p[-1]
    t += 1
    p = p[:-1]
print "%d %d" % (a,b)
