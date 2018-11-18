subs = {
    "at":"@",
    "and":"&",
    "one":"1",
    "won":"1",
    "to":"2",
    "too":"2",
    "two":"2",
    "for":"4",
    "four":"4",
    "bea":"b",
    "be":"b",
    "bee":"b",
    "sea":"c",
    "see":"c",
    "eye":"i",
    "oh":"o",
    "owe":"o",
    "are":"r",
    "you":"u",
    "why":"y"
}

for l in range(int(raw_input())):
    l = raw_input().split()
    st = ""
    for w in l:
        s = 0
        e = len(w)
        while s < e:
            for i in range(e, s - 1, -1):
                wo = w[s:i]
                wl = wo.lower()
                if wl in subs:
                    st += subs[wl].upper() if wo[0].isupper() else subs[wl]
                    s = i
                    break
                if s == i:
                    st += w[i]
                    s += 1

        st += " "
print st
