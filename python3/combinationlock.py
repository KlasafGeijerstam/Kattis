while True:

    t = 0

    c = [int(i) * 9 for i in input().split()]

    for i in c: t = t + i

    if t == 0: break

    print(720 + (c[0]-c[1]+360) % 360 + 360 + (c[2]-c[1]+360) % 360 + (c[2]-c[3]+360) % 360)