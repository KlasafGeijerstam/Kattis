_, m = map(int, input().split())
fishies = sorted(map(int, input().split()))
monger = sorted([list(map(int, input().split())) for _ in range(m)], key=lambda x: x[1])
monies = 0
while fishies and monger:
    monies += monger[-1][1] * fishies[-1]
    monger[-1][0] -= 1
    if monger[-1][0] == 0:
        del monger[-1]
    del fishies[-1]
print(monies)
