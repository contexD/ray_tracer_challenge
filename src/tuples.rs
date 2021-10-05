use duplicate::duplicate;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Tuple {
    value: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct Point {
    value: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct Vector {
    value: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub value: Vec<f32>,
}

pub trait Value {
    fn value(&self) -> &Vec<f32>;
}

impl Tuple {
    pub fn new(value: Vec<f32>) -> Self {
        Tuple { value }
    }
}

impl Point {
    pub fn new(value: Vec<f32>) -> Self {
        let mut p = value.to_vec();
        p.push(1.0);
        Point { value: p }
    }
}

impl Vector {
    pub fn new(value: Vec<f32>) -> Self {
        let mut v = value.to_vec();
        v.push(0.0);
        Vector { value: v }
    }

    pub fn magnitude(&self) -> f32 {
        self.value.iter().map(|x| x.powi(2)).sum::<f32>().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Self {
            value: self.value.iter().map(|x| x / magnitude).collect(),
        }
    }
}

impl Color {
    pub fn new(value: Vec<f32>) -> Self {
        Color { value }
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
    v1.value()
        .iter()
        .zip(v2.value().iter())
        .map(|(x, y)| x * y)
        .sum::<f32>()
}

pub fn cross(v1: &Vector, v2: &Vector) -> Vector {
    let mut f1: Vec<f32> = v1.value().to_vec();
    let mut f2: Vec<f32> = v2.value().to_vec();
    let mut result: Vec<f32> = Vec::with_capacity(4);

    f1.pop();
    f2.pop();
    f1.rotate_left(1);
    f2.rotate_right(1);

    for (index, _value) in f1.iter().enumerate() {
        result.push(f1[index] * f2[index] - f1[(index + 1) % 3] * f2[(index + 2) % 3]);
    }

    result.push(0.0);

    Vector { value: result }
}

#[duplicate(
   tuple_type;
   [ Tuple ];
   [ Point ];
   [ Vector ];
   [ Color ];
)]
impl Value for tuple_type {
    fn value(&self) -> &Vec<f32> {
        &self.value
    }
}

#[duplicate(
   tuple_type;
   [ Tuple ];
   [ Point ];
   [ Vector ];
   [ Color ];
)]
impl PartialEq for tuple_type {
    fn eq(&self, other: &Self) -> bool {
        let self_iter = self.value.iter();
        let other_iter = other.value().iter();

        let unequal_tuple = self_iter.zip(other_iter).find(|(x, y)| !equal(x, y));

        match unequal_tuple {
            None => true,
            _ => false,
        }
    }
}

impl Eq for Tuple {}
impl Eq for Point {}
impl Eq for Vector {}

#[duplicate(
   Lhs Rhs Output_type;
   #[
   Lhs_nested Rhs_nested Output_nested;
    [ Tuple ] [ Tuple ] [Tuple];
    [ Tuple ] [ Point ] [ Tuple ];
    [ Tuple ] [ Vector ] [ Tuple ];
    [ Point ] [ Tuple ] [ Tuple ];
    [ Point ] [ Vector ] [ Point ];
    [ Vector ] [ Vector ] [ Vector ];
    [ Vector ] [ Point ] [ Point ];
    [ Vector ] [ Tuple ] [ Tuple ];
    [ Color ] [ Color ] [ Color ];
   ][
    [ Lhs_nested ] [ Rhs_nested ] [ Output_nested ];
    [ Lhs_nested ] [ &Rhs_nested ] [ Output_nested ];
    [ &Lhs_nested ] [ Rhs_nested ] [ Output_nested ];
    [ &Lhs_nested ] [ &Rhs_nested ] [ Output_nested ];
   ]
)]
impl Add<Rhs> for Lhs {
    type Output = Output_type;

    fn add(self, rhs: Rhs) -> Self::Output {
        let lhs_iter = self.value.iter();
        let rhs_iter = rhs.value().iter();

        Self::Output {
            value: lhs_iter.zip(rhs_iter).map(|(&x, &y)| x + y).collect(),
        }
    }
}

#[duplicate(
   Lhs Rhs Output_type;
   #[ Lhs_inner Rhs_inner Output_inner;
   [ Point ] [ Point ] [ Vector ];
   [ Point ] [ Vector ] [ Point ];
   [ Vector ] [ Vector ] [ Vector ];
   [ Color ] [ Color ] [ Color ];
   ]
   [
   [ Lhs_inner ] [ Rhs_inner ] [ Output_inner];
   [ &Lhs_inner ] [ &Rhs_inner ] [ Output_inner];
   [ Lhs_inner ] [ &Rhs_inner ] [ Output_inner];
   [ &Lhs_inner ] [  Rhs_inner ] [ Output_inner];
   ]
)]
impl Sub<Rhs> for Lhs {
    type Output = Output_type;

    fn sub(self, rhs: Rhs) -> Self::Output {
        let lhs_iter = self.value.iter();
        let rhs_iter = rhs.value().iter();

        Self::Output {
            value: lhs_iter.zip(rhs_iter).map(|(&x, &y)| x - y).collect(),
        }
    }
}

#[duplicate(
   tuple_type;
   [ Tuple ];
   [ Vector ];
)]
impl Neg for tuple_type {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            // multiply x by (-1.0); -x results in the last component
            // being equal to -0.0
            value: self.value.iter().map(|&x| x * (-1.0)).collect(),
        }
    }
}

#[duplicate(
   tuple_type;
   [ Tuple ];
   [ Vector ];
   [ Color ];
)]
impl Mul<f32> for tuple_type {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            value: self.value.iter().map(|&x| x * rhs).collect(),
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::Output {
            value: self
                .value
                .iter()
                .zip(rhs.value.iter())
                .map(|(x, y)| x * y)
                .collect(),
        }
    }
}

#[duplicate(
   tuple_type;
   [ Tuple ];
   [ Vector ];
)]
impl Div<f32> for tuple_type {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            value: self.value.iter().map(|&x| x / rhs).collect(),
        }
    }
}

fn equal(x: &f32, y: &f32) -> bool {
    if (x - y).abs() < f32::EPSILON {
        return true;
    }
    return false;
}
