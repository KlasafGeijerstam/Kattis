class Box:
    def __init__(self, x):
        x1, y1, x2, y2, t = x.split()
        self.x1 = float(x1)
        self.y1 = float(y1)
        self.x2 = float(x2)
        self.y2 = float(y2)
        self.size = t

    def isin(self, x, y):
        return x >= self.x1 and x <= self.x2 and y >= self.y1 and y <= self.y2


n = int(input())
while n != 0:
    b = [Box(input()) for _ in range(n)]
    for _ in range(int(input())):
        x, y, t = input().split()
        pos = 'floor'
        for l in b:
            if l.isin(float(x), float(y)):
                pos = l.size
                break
        print('%s %s' % (t, 'correct' if t == pos else pos))
    print()
    n = int(input())
