if __name__ == '__main__':
    c, s = raw_input().split()
    if c == 'E':
        f = ""
        i = 0
        while i != len(s):
            j = 0
            for k in range(i, len(s)):
                if s[i] == s[k]:
                    j += 1
                else:
                    break
            f += s[i] + ("" if j == 0 else str(j))
            i += j
        print f
    else:
        f = ""
        i = 0
        while i < len(s):
            k = 1
            if i < len(s) and 47 < ord(s[i + 1]) < 58:
                f += s[i] * int(s[i + 1])
                i += 1
            else:
                f += s[i]
            i += 1
    print f
