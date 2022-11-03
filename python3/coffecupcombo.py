input()
coffe = 0
awake = 0
for d in input():
    if d == '1':
        coffe = 2
        awake += 1
    elif coffe > 0:
        awake += 1
        coffe -= 1
print(awake)