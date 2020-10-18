use std::cmp::Ordering;
use std::ops::*;

pub trait PointTrait: Copy + Clone + PartialOrd
                     + Add<Output = Self> + Sub<Self, Output = Self> + AddAssign 
                     + SubAssign + Zero + Mul<Self, Output = Self>
                     + MulAssign<Self> + DivAssign<Self> {}
impl<T> PointTrait for T where T: Copy + Clone + PartialOrd
                                + Add<Output = T> + Sub<T, Output = T> + AddAssign
                                + SubAssign + Zero + Mul<T, Output = T>
                                + MulAssign<T> + DivAssign<T> {}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Hash)]
pub struct Point<T: PointTrait> {
   pub x: T,
   pub y: T,
}

#[allow(dead_code, unused)]
impl<T: PointTrait> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn cross2(&self, a: &Self, b: &Self) -> T {
        (*a - *self).cross(&(*b - *self))
    }

    pub fn dot(&self, o: &Self) -> T {
        self.x * o.x + self.y * o.x
    }

}

#[allow(dead_code, unused)]
impl Point<f64> {
    pub fn dist(&self) -> f64 {
        f64::sqrt(self.dist2())
    }

    pub fn dist2(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    pub fn dist_between(&self, other: &Point<f64>) -> f64 {
        f64::sqrt(self.dist_between2(other))
    }

    pub fn dist_between2(&self, other: &Point<f64>) -> f64 {
        (self.x - other.x).powf(2.0)
        + (self.y - other.y).powf(2.0)
    }
}

impl<T: PointTrait> Add for Point<T> {
    type Output = Point<T>; 
    fn add(mut self, other: Point<T>) -> Self {
        self.x += other.x; 
        self.y += other.y;
        self
    }
}

impl<T: PointTrait> Sub for Point<T> {
    type Output = Point<T>; 
    fn sub(mut self, other: Point<T>) -> Self {
        self.x -= other.x; 
        self.y -= other.y;
        self
    }
}

impl<T: PointTrait> Eq for Point<T> {}

impl<T: PointTrait> Ord for Point<T> {
    fn cmp(&self, other: &Point<T>) -> Ordering {
        match self.x.partial_cmp(&other.x).unwrap() {
            Ordering::Equal => {
                self.y.partial_cmp(&other.y).unwrap()        
            },
            r@_ => {
                r   
            },
        }
    }
}

impl<T: PointTrait> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(mut self, f: T) -> Self {
        self.x *= f;
        self.y *= f;
        self
    }
}

impl<T: PointTrait> Div<T> for Point<T> {
    type Output = Self;
    fn div(mut self, f: T) -> Self {
        self.x /= f;
        self.y /= f;
        self
    }
}


pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i128 {
    fn zero() -> Self {
        0
    }
}
