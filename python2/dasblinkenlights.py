if __name__ == '__main__':
    r = map(int, raw_input().split())
    for i in range(1, r[2]+1):
        if i % r[0] == 0 and i % r[1] == 0:
            print "yes"
            exit(0)
print "no"
