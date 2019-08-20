from math import sqrt
ps = [True for _ in range(10001)]
ps[1] = False
i = 2
while i < sqrt(10001):
    j = i*i
    if ps[i]:
        while j <= 10000:
            ps[j] = False
            j += i
    i += 1

for case, num in [tuple(map(int, input().split())) for _ in range(int(input()))]:
    k = set()
    happy = True
    onum = num
    while num != 1:
        num = sum([a*a for a in map(int, str(num))])
        if num in k:
            happy = False
            break
        k.add(num)

    print('{0} {1} {2}'.format(case, onum, 'YES' if happy and ps[onum] else 'NO'))
