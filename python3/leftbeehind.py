while True:
    x = input().split()
    x[0] = int(x[0])
    x[1] = int(x[1])
    if x[0] == 0 and x[1] == 0:
        break
    if x[0] + x[1] == 13:
        print("Never speak again.")
    elif x[0] > x[1]:
        print("To the convention.")
    elif x[0] < x[1]:
        print("Left beehind.")
    else:
        print("Undecided.")
