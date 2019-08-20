from sys import stdin
from math import sqrt

def d(x1, y1, x2, y2):
    return sqrt((x1 - x2)**2 + (y1 - y2)**2)

l = [map(float, f.split()) for f in stdin]
xg, yg, xd, yd = l[0]
for x, y in l[1:]:
    if d(xg, yg, x, y) <= d(xd, yd, x, y) / 2:
        print(f'The gopher can escape through the hole at ({x},{y}).')
        exit()
print('The gopher cannot escape.')
