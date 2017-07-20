n = int(input())
for e in range(n):
    x = input().split(',')
    p = 0
    i = len(x)-1
    for v in x:
        if len(v) > 0:
            p += int(v)*pow(60, i)
        i -= 1
    print(p)
