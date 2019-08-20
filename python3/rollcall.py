from sys import stdin


class Person:
    def __init__(self, n):
        self.fn = n[0]
        self.ln = n[1]

    def __lt__(self, other):
        if self.ln != other.ln:
            return self.ln < other.ln
        return self.fn < other.fn


names = sorted([Person(l.split()) for l in stdin])
n = {}
for na in names:
    if na.fn not in n:
        n[na.fn] = 0
    n[na.fn] += 1
print('\n'.join([p.fn + ('' if n[p.fn] == 1 else ' {0}'.format(p.ln)) for p in names]))
