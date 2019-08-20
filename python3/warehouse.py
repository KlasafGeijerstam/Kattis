class Ware:
    def __init__(self, n):
        self.name = n[0]
        self.amount = int(n[1])

    def __lt__(self, other):
        if self.amount != other.amount:
            return self.amount > other.amount
        return self.name < other.name

for _ in range(int(input())):
    n = int(input())
    p = {}
    for _ in range(n):
        w = Ware(input().split())
        if w.name not in p:
            p[w.name] = w
        else:
            p[w.name].amount += w.amount

    ps = sorted(p.values())
    print(len(ps))
    for c in ps:
        print(f'{c.name} {c.amount}')
