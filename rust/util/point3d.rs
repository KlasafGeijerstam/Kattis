use std::ops::*;
#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[allow(unused)]
impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D {
            x,
            y,
            z
        }
    }

    pub fn norm(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        *self / n
    }

    pub fn cross(self, o: Self) -> Self {
        Point3D {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    pub fn dot(self, o: Self) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z 
    }
}

impl Mul<f64> for Point3D {
    type Output = Self;
    fn mul(mut self, f: f64) -> Self {
        self.x *= f;
        self.y *= f;
        self.z *= f;

        self
    }
}

impl Div<f64> for Point3D {
    type Output = Self;
    fn div(mut self, f: f64) -> Self {
        self.x /= f; 
        self.y /= f; 
        self.z /= f; 

        self
    }
}

impl Add<Point3D> for Point3D {
    type Output = Self;
    fn add(mut self, f: Self) -> Self {
        self.x += f.x;
        self.y += f.y;
        self.z += f.z;

        self
    }
}

impl Add<f64> for Point3D {
    type Output = Self;
    fn add(mut self, f: f64) -> Self {
        self.x += f;
        self.y += f;
        self.z += f;

        self
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Self;
    fn sub(mut self, f: Self) -> Self {
        self.x -= f.x;
        self.y -= f.y;
        self.z -= f.z;

        self
    }
}

impl Sub<f64> for Point3D {
    type Output = Self;
    fn sub(mut self, f: f64) -> Self {
        self.x -= f;
        self.y -= f;
        self.z -= f;

        self
    }
}

impl From<(f64, f64, f64)> for Point3D {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
    }
}
