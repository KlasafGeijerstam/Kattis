import sys
c = 1
while True:
    try:
        nums = [int(input()) for _ in range(int(input()))]
        print 'Case {0}:'.format(c)
        c += 1
        sums = set()
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                if i != j:
                    sums.add(nums[i] + nums[j])

        for q in [int(input()) for _ in range(int(input()))]:
            s = 0
            d = 999999999
            for n in sums:
                if abs(q - n) < d:
                    d = abs(q - n)
                    s = n
            print 'Closest sum to {0} is {1}.'.format(q, s)
    except EOFError:
        break
