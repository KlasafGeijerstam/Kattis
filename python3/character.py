def f(p):
    if p < 2:
        return 1
    return p * f(p - 1)

def c(p, k):
    return f(p) / (f(k) * f(p - k))

tot = 0
p = int(input())
for i in range(2,p+1):
    tot = tot + c(p, i)

print(int(tot))
