n = int(input())
p = {}
for c in [input() for _ in range(n)]:
    if c not in p:
        p[c] = 0
    p[c] += 1

ps = sorted([(v, k) for k, v in p.items()])
n = ps[0][0]
for c in ps:
    if c[0] > n:
        break
    print(c[1])
