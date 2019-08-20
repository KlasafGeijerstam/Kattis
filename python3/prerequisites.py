from sys import stdin

while True:
    l = list(map(int, input().split()))
    if len(l) == 1:
        break
    k, m = l[0], l[1]
    
    fail = False
    courses = set(input().split())
    for j in range(m):
        n = input().split()
        if not fail and len(courses.intersection(n[2:])) < int(n[1]):
            fail = True
    print('no' if fail else 'yes')
