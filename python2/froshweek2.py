from __future__ import print_function

if __name__ == '__main__':
    n, m = map(int, raw_input().split())
    t = map(int, raw_input().split())
    i = map(int, raw_input().split())
    i.sort(reverse=True)
    t.sort(reverse=True)
    j = 0
    w = 0
    k = 0
    while k < (len(i)):
        if j >= len(t):
            break
        if i[k] >= t[j]:
            w += 1
            k += 1
        j += 1
print(w) 
