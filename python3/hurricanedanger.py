from math import sqrt
class City:
    def __init__(self, l, x1, y1, x2, y2):
        self.name = l[0]
        self.x = int(l[1])
        self.y = int(l[2])
        self.d = 0
        self.distance_to(x1, y1, x2, y2)

    def distance_to(self, x1, y1, x2, y2):
        x0, y0 = self.x, self.y
        self.d = abs((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1) / sqrt((y2 - y1) ** 2 + (x2 - x1) ** 2)

    def __lt__(self, other):
        return self.d < other.d

for _ in range(int(input())):
    x1, y1, x2, y2 = map(int, input().split())
    cities = sorted([City(input().split(), x1, y1, x2, y2) for _ in range(int(input()))])
    m = min([c.d for c in cities])
    print(' '.join([c.name for c in cities if c.d == m]))
