from sys import stdin
trees, tot = {}, 0
for l in stdin:
    l = l.strip()
    if l not in trees:
        trees[l] = 0
    trees[l] += 1
    tot += 1
for t in sorted(trees):
    print('{0} {1}'.format(t, 100 * trees[t] / tot))
