y = int(raw_input())
b = 2018
m = 3
while b != y:
    m += 26
    while m >= 12:
        b += 1
        m -= 12
    if b == y:
        break
    elif b > y:
        print "no"
        exit(0)
print "yes"
