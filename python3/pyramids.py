n = int(input())
for i in range(1000):
    a = (1 + 2 * i) ** 2
    if a > n:
        break
    n -= a
    
print(i)