def fac(pw):
    if pw == 0:
        return 1
    return pw*fac(pw-1)

def non(u,k):
    return fac(u) // (fac(k)*fac(u-k))

def calc(j, m):
    tot = 2**j
    for i in range(0, m):
        tot -= non(j, i)
    return tot / (2**j)

t = int(input())

for tk in range(t):
    k = list(map(int, input().split()))

    if k[2] >= (k[0] // 2) + 1 - (1 if k[0] % 2 == 0 else 0) or (k[2] + k[1] == k[0] and k[1] < (k[0] // 2) + 1):
        print("RECOUNT!")
    elif k[1] >= (k[0] // 2) + 1 or calc(k[0] - k[1] - k[2], ((k[0] // 2) + 1) - k[1])*100 > k[3]:
        print("GET A CRATE OF CHAMPAGNE FROM THE BASEMENT!")
    else:
        print("PATIENCE, EVERYONE!")
