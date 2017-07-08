from sys import stdin

for line in stdin:
    line = int(line)
    ai = 1+ line//3
    while (ai*ai*ai) > line:
        ai = int((2*ai + line // int(ai*ai))//3)
    if abs(line - pow(ai, 3)) > abs(line - pow(ai+1, 3)):
        ai = ai + 1
    print(ai)
