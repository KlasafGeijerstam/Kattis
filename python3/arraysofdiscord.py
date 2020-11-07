def is_sorted(x):
    a = int(''.join(x[0]))
    for b in x[1:]:
        bint = int(''.join(b))
        if bint < a:
            return False
        a = bint
    return True


n = int(input())
nums = [n for n in input().split()]
orig_nums = [int(n) for n in nums]
nums = [list(n) for n in nums]
for number in nums:
    for pos, digit in enumerate(number):
        original = digit
        for i in range(ord('0'), ord('9') + 1):
            c = chr(i)

            if pos == 0 and c == '0' and len(number) > 1:
                continue

            number[pos] = c
            if not is_sorted(nums):
                print(' '.join(map(lambda x: ''.join(x), nums)))
                exit(0)
        number[pos] = original
print("impossible")