from __future__ import print_function
from sys import stdin

for l in stdin:
    r = [int(x) for x in l.split()]
    mv = max(r)
    v = [False for i in range(mv + 1)]
    try:
        for m1 in r:
            for m2 in r:
                if m1 == m2 or m1 - m2 < 1:
                    continue
                if not v[m1 - m2]:
                    v[m1 - m2] = True
                else:
                    raise Exception()
    except:
        print("not a ruler")
        continue

    mis = [str(i) for i in range(1, mv + 1) if not v[i]]

    if mis:
        print("missing %s" % " ".join(mis))
    else:
        print("perfect")