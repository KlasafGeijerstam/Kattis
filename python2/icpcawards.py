import sys
raw_input()
x = []
for i in sys.stdin:
    i = i.rstrip()
    if not i.split()[0] in x:
        x.append(i.split()[0])
        print(i)
    if len(x) == 12:
    break
