x = input().split()
bob = (int(x[1]) - int(x[0]))*int(x[2])
alice = 0
while alice <= bob:
    alice = alice + int(x[4])
print(int(alice / int(x[4]) + int(x[3])))
