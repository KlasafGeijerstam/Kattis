for _ in range(int(raw_input())):
    c = raw_input()
    cnt = int(raw_input())
    l = raw_input()[1:-1].split(',')
    li = 0
    ri = 0
    r = False
    for com in c:
        if com == 'R':
            r = not r
            t = li
            li = ri
            ri = t
        else:
            li += 1
    if li > len(l) - ri or ('D' in c and cnt == 0):
        print "error"
    else:
        if r:
            l.reverse()
        print "[" + (",".join(l[li:len(l) - ri])) + "]"
