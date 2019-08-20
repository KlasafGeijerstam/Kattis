p = input()
w = []
for i in range(1, len(p)-2):
    for j in range(i+1, len(p)):
        w.append(''.join(reversed(p[:i])) + ''.join(reversed(p[i:j])) + ''.join(reversed(p[j:])))

print(sorted(w)[0])
