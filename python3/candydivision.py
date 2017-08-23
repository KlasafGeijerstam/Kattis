import math

l = int(input())
p = []

for i in range(0, int(math.sqrt(l))):
    if l % (i + 1) == 0:
        print(str(i) + " ", end='')
        if l//(i+1)-1 != i:
            p.append(l//(i+1)-1)
p.reverse()
for i in p:
    print(str(i) + " ", end='')
