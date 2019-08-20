from sys import stdin
dig = [1,1,2,6,4,2,2,4,2,8]

def l(a):
    if a < 10:
        return dig[a]
    if ((a//10)%10)%2 == 0:
        return (6*l(a//5)*dig[a%10]) % 10
    else:
        return (4*l(a//5)*dig[a%10]) % 10
    return 0

for n in map(int, stdin):
    if n == 0:
        break
    print(l(n))
