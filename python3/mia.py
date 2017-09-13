line = list(map(int, input().split()))
while line[0] + line[1] + line[2] + line[3] > 0:
    p1 = 0
    p2 = 0
    if line[0] + line[1] == 3:
        p1 = 1000
    if line[2] + line[3] == 3:
        p2 = 1000

    if p1 == 0 and line[0] == line[1]:
        p1 = 100 + line[0]
    if p2 == 0 and line[2] == line[3]:
        p2 = 100 + line[2]

    if p1 == 0:
        p1 = int(str(max(line[0], line[1])) + str(min(line[0], line[1])))
    if p2 == 0:
        p2 = int(str(max(line[2], line[3])) + str(min(line[2], line[3])))

    if p1 == p2:
        print("Tie.")
    else:
        print("Player 1 wins." if p1 > p2 else "Player 2 wins.")
    line = list(map(int, input().split()))
