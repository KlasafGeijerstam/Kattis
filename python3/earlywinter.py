n, t = map(int, input().split())
p = list(map(int, input().split()))
for i, v in enumerate(p):
    if t >= v:
        print("It hadn't snowed this early in {0} years!".format(i))
        exit(0)
print("It had never snowed this early!")
