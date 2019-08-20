for _ in range(int(raw_input())):
    k, x = raw_input().split()
    try:
        o = int(x, 8)
    except ValueError:
        o = 0
    try:
        d = int(x)
    except ValueError:
        d = 0
    try:
        h = int(x, 16)
    except ValueError:
        h = 0
    print "%s %d %d %d" % (k, o, d, h)
