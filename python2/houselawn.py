from __future__ import print_function

if __name__ == '__main__':
    l, m = map(int, raw_input().split())
    names = []
    lowest = 10000000
    for i in range(m):
        a = raw_input().split(',')
        name = a[0]
        p, c, t, r = map(float, a[1:])
        if ((c*t)/(t+r)) * 10080 >= l:
            if p < lowest:
                lowest = p
                names = [name]
            elif p == lowest:
                names.append(name)
    if len(names) == 0:
        print("no such mower")
    else:
        for i in names:
            print(i)

