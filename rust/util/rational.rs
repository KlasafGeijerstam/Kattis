use std::ops::*;

pub type RatType = i64;

#[allow(unused)]
#[derive(Debug)]
pub struct Rational {
    pub num: RatType,
    pub den: RatType, 
}

#[allow(unused)]
impl Rational {
    pub fn new(mut num: RatType, mut den: RatType) -> Self {
        let mut r = Rational {
            num,
            den
        };
        r.clamp();
        r
    }

    pub fn fix(n1: Self, n2: Self) -> (RatType, RatType, RatType) {
        if n1.den != n2.den {
            (n1.num * n2.den, n2.num * n1.den, n1.den * n2.den)
        } else {
            (n1.num, n2.num, n1.den)
        }
    }

    pub fn clamp(&mut self) {
        let gc = gcd(self.num, self.den);
        if gc.abs() > 1 {
            self.num /= gc;
            self.den /= gc;
        }

        if self.den < 0 {
            self.den *= -1;
            self.num *= -1;
        }
    }
}

impl Add for Rational {
    type Output=Self;

    fn add(self, other: Self) -> Self {
        let (n1, n2, d) = Rational::fix(self, other);
        Rational::new(n1 + n2, d)
    }
}

impl Mul for Rational {
    type Output=Self;

    fn mul(self, other: Self) -> Self {
        Rational::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Rational {
    type Output=Self;

    fn div(self, other: Self) -> Self {
        Rational::new(self.num * other.den, self.den * other.num)
    }
}

impl Sub for Rational {
    type Output=Self;

    fn sub(self, other: Self) -> Self {
        let (n1, n2, d) = Rational::fix(self, other);
        Rational::new(n1 - n2, d)
    }
}

#[allow(unused)]
pub fn gcd(a: RatType, b: RatType) -> RatType {
    if b != 0 { gcd(b, a % b) } else { a }
}

#[allow(unused)]
pub fn lcd(a: RatType, b: RatType) -> RatType {
    (a / gcd(a, b)) * b
}
