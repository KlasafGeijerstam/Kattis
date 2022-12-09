
n = int(input())
for _ in range(n):
    n1 = int(''.join(input().split()))
    n2 = int(''.join(input().split()))
    print(' '.join(str(n1 + n2)))