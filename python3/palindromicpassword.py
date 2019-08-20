def pal(i):
    s = str(i)

    if s[0] == '0' or len(s) != 6:
        return False

    for k in range(3):
        if s[k] != s[- (1 + k)]:
            return False
    return True


for i in [int(input()) for _ in range(int(input()))]:
    d = 0
    while not pal(i + d) and not pal(i - d):
        d += 1
    print(i - d if pal(i - d) else i + d)
