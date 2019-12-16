import sys
for l in [k for k in sys.stdin][:-1]:
    l = l[:-1]
    if l == '1':
        print(1)
    elif len(l) == 1:
        print(2)
    elif len(l) < 10:
        print(3)
    else:
        print(4)
