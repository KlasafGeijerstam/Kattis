for i in range(0, int(input())):
    v = input().split()
    y = (int(v[0]) - int(v[1]))/2
    x = int(v[0]) - y
    if (int(v[0])-int(v[1])) % 2 != 0 or x < 0 or y < 0:
        print("impossible")
    else:
        print(str(int(x)) + " " + str(int(y)))
