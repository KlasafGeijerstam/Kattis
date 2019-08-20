for _ in range(int(input())):
    num = list(map(int, input().split()))
    nums = num[1:]
    l = [nums[0]]
    s = 0
    for n in nums[1:]:
        j = 0
        while j < len(l) and l[j] <= n:
            j += 1
        l.insert(j, n)
        s += (len(l) - 1) - j
    print('{0} {1}'.format(num[0], s))
