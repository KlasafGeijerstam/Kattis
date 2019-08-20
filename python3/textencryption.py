n = int(input())
while n != 0:
    s = input().replace(' ', '').upper()
    k = ['a' for _ in range(len(s))]
    i = 0
    b = 0
    for c in s:
        k[i] = c
        i += n
        if i >= len(s):
            i = b + 1
            b += 1
    print(''.join(k))
    n = int(input())
