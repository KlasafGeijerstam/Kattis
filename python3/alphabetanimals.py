import sys

def search(ani, w, ignore):
    for i,a in enumerate(ani):
        if i == ignore:
            continue
        if w[-1] == a[0]:
            return True
    return False

startanimal = input()
n = int(input())

animals = [input() for _ in range(n)]

for i,a in enumerate(animals):
    if a[0] == startanimal[-1] and not search(animals, a, i):
        print(a + "!")
        sys.exit(0)

for a in animals:
    if a[0] == startanimal[-1]:
        print(a)
        sys.exit(0)
print('?')
