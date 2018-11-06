s,e = map(int, raw_input().split())
c = 0
for j in range(s, e + 1):
    if str(j).find("0") == -1 and all(j % int(x) == 0 for x in str(j)) and len(set(str(j))) == len(str(j)):
        c += 1
print c
