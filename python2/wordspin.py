def m(a, b):
    return (ord(a) - 97) - (ord(b) - 97)

s1, s2 = raw_input().split()
#dist = [(ord(y) - 97) - (ord(x) - 97) for (x, y) in zip(a, b)]
prev = m(s1[0], s2[0])
inc = abs(prev)
for i in range(1, len(s1)):
    a = m(s1[i], s2[i])
    if 0 < a > prev > 0 or 0 > a < prev < 0:
        inc += abs(a-prev)
    elif (a >= 0 >= prev) or (a <= 0 <= prev):
        inc += abs(a)
    prev = a

print inc
