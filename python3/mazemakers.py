h, w = map(int, input().split())
while h + w != 0:
    m = [input() for _ in range(h)]
    v = [[True for _ in range(w)] for _ in range(h)]
    ends = []
    for i in range(w):
        if m[0][i] in '04152637':
            ends.append((i, 0))
        elif m[-1][i] in '048C159D':
            ends.append((i, h - 1))

    for i in range(h):
        if m[i][0] in '048C26AE':
            ends.append((0, i))
        elif m[i][-1] in '08192A3B':
            ends.append((w - 1, i))
    if len(ends) == 1:
        ends = ends * 2
    s = [ends[0], ends[0]]
    rp = 0
    gf = 0
    while s:
        lp = s.pop()
        x, y = s.pop()

        v[y][x] = False
        b = [x == '0' for x in bin(int(m[y][x], 16))[2:].rjust(4, '0')]

        if b[0] and y - 1 >= 0:
            if v[y - 1][x]:
                s.append((x, y - 1))
                s.append((x, y))
            elif (x, y - 1) != lp:
                gf += 1
        if b[1] and x + 1 < w:
            if v[y][x + 1]:
                s.append((x + 1, y))
                s.append((x, y))
            elif (x + 1, y) != lp:
                gf += 1
        if b[2] and y + 1 < h:
            if v[y + 1][x]:
                s.append((x, y + 1))
                s.append((x, y))
            elif (x, y + 1) != lp:
                gf += 1
        if b[3] and x - 1 >= 0:
            if v[y][x - 1]:
                s.append((x - 1, y))
                s.append((x, y))
            elif (x - 1, y) != lp:
                gf += 1

        if (x, y) == ends[1]:
            rp += 1

    if rp == 0:
        print('NO SOLUTION')
    elif any([any(l) for l in v]):
        print('UNREACHABLE CELL')
    elif gf > 0:
        print('MULTIPLE PATHS')
    else:
        print('MAZE OK')

    h, w = map(int, input().split())
