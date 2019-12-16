input()
app = {}
freq = {}
for i, t in enumerate(input().split()):
    if not t in freq:
        freq[t] = 0
    if not t in app:
        app[t] = i
    freq[t] += 1

for i in sorted(freq, key=freq.get, reverse=True):
    print(" ".join([i] * freq[i]), end=' ')
print()
