for _ in range(int(raw_input())):
    s = raw_input()
    k = ""
    a = 0
    for c in range(len(s)-1, -1, -1):
        if a % 2 != 0:
            c = int(s[c])*2
            k = (str(c) if len(str(c)) == 1 else str(sum(map(int, str(c))))) + k
        else:
            k = str(s[c]) + k
        a += 1
    print "PASS" if (sum(map(int, k)) % 10 == 0) else "FAIL"
