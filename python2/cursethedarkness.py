from math import sqrt
for _ in range(int(raw_input())):
    bx,by = map(float, raw_input().split())
    f = False
    for _ in range(int(raw_input())):
        cx,cy = map(float, raw_input().split())
        if sqrt((bx - cx)**2 + (by - cy)**2) <= 8:
            f = True
    print "light a candle" if f else "curse the darkness"
