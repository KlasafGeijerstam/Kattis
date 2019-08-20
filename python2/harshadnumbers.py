i = int(raw_input())
while True:
    if i % sum([int(d) for d in str(i)]) == 0:
        print i
        break
    i += 1
