use super::EPSILON;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple(pub f32, pub f32, pub f32, pub f32);

impl Tuple {
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Tuple(x, y, z, 1.0)
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Tuple(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }

    pub fn magnitude(&self) -> f32 {
        let Tuple(x, y, z, w) = self;

        (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let Tuple(x, y, z, w) = self;
        let magnitude = self.magnitude();

        Tuple(x / magnitude, y / magnitude, z / magnitude, w / magnitude)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, Tuple(x, y, z, w): &Self) -> bool {
        let Tuple(a, b, c, d) = self;
        equal(a, x) && equal(b, y) && equal(c, z) && equal(d, w)
    }
}

impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let Tuple(a, b, c, d) = self;
        let Tuple(x, y, z, w) = other;

        Self(a + x, b + y, c + z, d + w)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let Tuple(a, b, c, d) = self;
        let Tuple(x, y, z, w) = other;

        Self(a - x, b - y, c - z, d - w)
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Tuple(x, y, z, w) = self;

        Self(-x, -y, -z, -w)
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let Tuple(x, y, z, w) = self;

        Self(x * rhs, y * rhs, z * rhs, w * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let Tuple(x, y, z, w) = self;

        Self(x / rhs, y / rhs, z / rhs, w / rhs)
    }
}

fn equal(x: &f32, y: &f32) -> bool {
    if (x - y).abs() < EPSILON {
        return true;
    }
    return false;
}
