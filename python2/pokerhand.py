x = {'A':0, '2':0, '3':0, '4':0, '5':0, '6':0, '7':0, '8':0, '9':0, 'T':0, 'J':0, 'Q':0, 'K':0}
for j in raw_input().split():
    x[j[0]] += 1;
print max(x.values())
