l1 = [x for x in input()]
l2 = [x for x in input()]
s = l1.count('S') * l2.count('S')
for j in range(s):
    print('S(', end='')
print('0', end='')
for j in range(s):
    print(')', end='')
print()