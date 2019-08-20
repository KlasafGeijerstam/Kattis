from math import sqrt
n, m, k = map(int, input().split())
plots = sorted(map(int, input().split()))
houses = list(map(int, input().split()))
houses.extend(map(lambda x: sqrt(2*(int(x)**2)) / 2, input().split()))
houses.sort(reverse=True)

m = 0
for p in plots:
    for h in houses:
        if h < p:
            houses.remove(h)
            m += 1
            break
print(m)
