from sys import stdin
previous_result = 1

class N:
    def __init__(self, v):
        self.v = int(v)

    def __add__(self, other):
        global previous_result
        return self.v + other.v - previous_result
    
    def __sub__(self, other):
        global previous_result
        return (self.v - other.v) * previous_result 
    
    def __mul__(self, other):
        global previous_result
        return int((self.v * other.v) ** 2)
    
    def __truediv__(self, other):
        if self.v % 2 == 0:
            return self.v // 2
        return (self.v + 1) // 2

for l in stdin.readlines()[1:]:
    a, op, b = l.strip().split()
    a, b = (N(a), N(b))
    if op == '+':
        previous_result = a + b
    elif op == '-':
        previous_result = a - b
    elif op == '*':
        previous_result = a * b
    else:
        previous_result = a / b
    print(previous_result)
