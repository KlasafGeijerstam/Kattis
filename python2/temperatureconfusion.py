import fractions
x = fractions.Fraction(raw_input())
x = (x - 32) / fractions.Fraction(9,5)
print "%d/%d" % (x.numerator,x.denominator)