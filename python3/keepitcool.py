import sys
n,m,s,d = map(int, input().split())

slots = [[True] * int(i) for i in input().split()]
additions = [0] * s
for i,s in enumerate(slots):
    if len(s) == 0:
        b = d if n > d else n
        s += [False] * b
        additions[i] += b
        n -= b
        if n == 0:
            break
while n > 0:
    shortest = 99999999900
    sl = []
    isl = 0
    for i, s in enumerate(slots):
        if len(s) < shortest: 
            sl = s
            shortest = len(s)
            isl = i

    b = (d - len(sl)) if n > (d - len(sl)) else n
    sl += [False] * b
    additions[isl] += b
    n -= b
    
for s in slots:
    for c in reversed(s):
        if not c:
            break
        m -= 1
        if m == 0:
            print(" ".join(map(str, additions)))
            sys.exit(0) 
print('impossible')
