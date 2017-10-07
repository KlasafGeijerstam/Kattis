x = input().split()

if int(x[0]) + int(x[1]) == 0:
    print("Not a moose")
elif int(x[0]) != int(x[1]):
    print("Odd " + str(2*max(int(x[0]), int(x[1]))))
else:
print("Even " + str(2*int(x[0]))) 
