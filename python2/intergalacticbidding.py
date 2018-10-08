raw_input()
x = map(int, raw_input().split())
p = []
c = 0
for i in range(1, 366):
    if i in x:
        p.append(i);
    d = 0
    for j in p:
        d += (i + 1) - j
    if d > 19:
        c += 1
        p = []
if len(p) > 0:
    c += 1
print c
