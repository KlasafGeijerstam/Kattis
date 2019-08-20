from sys import stdin
ram = [0 for _ in range(1000)]
reg = [0 for _ in range(10)]

for i, l in enumerate(stdin):
    ram[i] = int(l)

ptr = 0
ops = 0
while True:
    ops += 1
    a, b, c = map(int, str(ram[ptr]).rjust(3, '0'))
    if a == 0 and reg[c] > 0:
        ptr = reg[b]
        continue
    elif a == 1:
        break
    elif a == 2:
        reg[b] = c
    elif a == 3:
        reg[b] = (reg[b] + c) % 1000
    elif a == 4:
        reg[b] = (reg[b] * c) % 1000
    elif a == 5:
        reg[b] = reg[c]
    elif a == 6:
        reg[b] = (reg[b] + reg[c]) % 1000
    elif a == 7:
        reg[b] = (reg[b] * reg[c]) % 1000
    elif a == 8:
        reg[b] = ram[reg[c]]
    elif a == 9:
        ram[reg[c]] = reg[b]
    ptr += 1
print ops
