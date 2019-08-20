def ssn(b,n):
    lst = []
    while n > 0:
        x = 0
        while True:
            x += 1
            if n - b**x < 0:
                x -= 1
                break

        k = 1
        while True: 
            k += 1
            if n - k * b**x < 0:
                k -= 1
                break
        lst.append(k*k)
        n -= k*b**x
    return sum(lst)

for i in range(int(input())):
    n, b, t = map(int, input().split())
    print("%s %d" % (n, ssn(b, t)))
