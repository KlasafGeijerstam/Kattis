w = list(input())
g = input()
h = 0
for a in g:
    if a in w:
        w = [k for k in w if k != a]
    else:
        h += 1
    if not w:
        break
print('WIN' if h < 10 else 'LOSE')