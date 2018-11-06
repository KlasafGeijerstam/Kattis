raw_input()
t = [i for i in map(int, raw_input().split())]
g = [t[0]]
for i in range(1,len(t)):
    if t[i] > g[-1]:
        g.append(t[i])

print len(g)
print " ".join(map(str, g))
