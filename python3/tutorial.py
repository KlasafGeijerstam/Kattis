from math import log2, factorial
m, n, t = map(int, input().split())
k = None
if t == 1:
    if n > 12:
        k = False
    else:
        k = factorial(n) <= m
elif t == 2:
    if n > 29:
        k = False
    else:
        k = 2**n <= m
elif t == 3:
    k = n**4 <= m
elif t == 4:
    k = n**3 <= m
elif t == 5:
    k = n**2 <= m
elif t == 6:
    k = n*log2(n) <= m
elif t == 7:
    k = n <= m
print('AC' if k else 'TLE')
