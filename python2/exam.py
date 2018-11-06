x = int(raw_input())
r = x
j = zip(raw_input(), raw_input())
c = 0
for i in range(x):
    for k in range(len(j)):
        if j[k][0] == j[k][1]:
            del j[k]
            c += 1
            r -= 1
            break
print len([i for i in j if i[0] != i[1]]) - r + c
