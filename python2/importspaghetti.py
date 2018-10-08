from __future__ import print_function
class C:
    def __init__(self, name):
        self.name = name
        self.dep = []


if __name__ == '__main__':

    n = int(raw_input())
    cods = {x:C(x) for x in raw_input().split()}

    for i in range(n):
        c, d = raw_input().split()
        for j in range(int(d)):
            for p in raw_input()[7:].split(','):
                cods[c].dep.append(p.strip())

    mi = 10000000
    op = []
    for i in cods.values():
        queue = [(i,[])]
        v = { x:True for x in cods }

        while len(queue) != 0:
            queue[0][1].append(queue[0][0].name)
            added = 0
            for d in queue[0][0].dep:
                if d == i.name:
                    if len(queue[0][1]) < mi:
                        mi = len(queue[0][1])
                        op = queue[0][1]
                    q = []
                    break
                elif v[d] and len(queue[0][1]) < mi:
                    queue.append((cods[d], queue[0][1][:]))
                    v[d] = False
            del queue[0]
    if len(op) > 0:
        for i in op:
            print(i, end=" ")
        print("")
    else:
        print("SHIP IT")

