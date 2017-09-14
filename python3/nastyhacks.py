for i in range(int(input())):
    l = list(map(int, input().split()))
    print("advertise" if l[0] < l[1] - l[2] else "do not advertise" if l[0] > l[1] - l[2] else "does not matter")
