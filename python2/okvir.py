from __future__ import print_function

def s(x):
    return '#' if x == '.' else '.'

def alt(s,l):
    if s == '#':
        return "".join(['#' if i % 2 == 0 else '.' for i in range(l)])
    else:
        return "".join(['.' if i % 2 == 0 else '#' for i in range(l)])

m, n = map(int, raw_input().split())
u, l, r, i = map(int, raw_input().split())
d = [raw_input() for _ in range(m)]

ps = '.'
ml = n + l + r
for _ in range(u):
    ps = s(ps)
    print(alt(ps, ml))

for lap in d:
    ps = s(ps)
    print(alt(ps, l) + lap + (alt(s(ps), r) if (len(lap) + l) % 2 != 0 else alt(ps, r)))

for _ in range(i):
    ps = s(ps)
    print(alt(ps, ml))
