from math import sqrt
def d(x):
    return sqrt((cx-x[0])**2 + (cy-x[1])**2) - x[2]
cx,cy,n = map(int, raw_input().split())
l = [d(map(int,raw_input().split())) for _ in range(n)] 
l.sort()
x = int(l[2])
print x if x > 0 else 0