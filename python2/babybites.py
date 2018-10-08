raw_input()
x = raw_input().split()
j = 1
for i in x:
    if i == "mumble":
        j += 1
    elif int(i) != j:
        print "something is fishy"
        exit()
    else:
        j += 1
print "makes sense"
