import sys


def sump(x):
    tot = 0
    for c in str(x):
        tot += int(c)
    return tot

n = sys.stdin.readline()[0:-1]
while True:

    t = sump(n)
    p = 11
    while sump(p*int(n)) != t:
        p += 1

    print(p)
    n = sys.stdin.readline()[0:-1]
    if int(n) == 0:
        break
