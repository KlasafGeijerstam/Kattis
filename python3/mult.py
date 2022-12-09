n = int(input())
target = None
for _ in range(n):
    num = int(input())
    if target is None:
        target = num
    elif num % target == 0:
        print(num)
        target = None
    