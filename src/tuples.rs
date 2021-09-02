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

pub trait Value {
    fn value(&self) -> &Vec<f32>;
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            value: vec![x, y, z, w],
        }
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point {
            value: vec![x, y, z, 1.0],
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector {
            value: vec![x, y, z, 0.0],
        }
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
   tuple_type other_type output_type;
   [ Tuple ] [ Self ] [Self];
   [ Tuple ] [ Point ] [ Tuple ];
   [ Tuple ] [ Vector ] [ Tuple ];
   [ Point ] [ Tuple ] [ Tuple ];
   [ Point ] [ Vector ] [ Point ];
   [ Vector ] [ Self ] [ Self ];
   [ Vector ] [ Point ] [ Point ];
   [ Vector ] [ Tuple ] [ Tuple ];
)]
impl Add<other_type> for tuple_type {
    type Output = other_type;

    fn add(self, other: other_type) -> Self::Output {
        let self_iter = self.value.iter();
        let other_iter = other.value().iter();

        Self::Output {
            value: self_iter.zip(other_iter).map(|(&x, &y)| x + y).collect(),
        }
    }
}

#[duplicate(
   tuple_type other_type output_type;
   [ Point ] [ Self ] [ Vector ];
   [ Point ] [ Vector ] [ Point ];
   [ Vector ] [ Self ] [ Self ];
)]
impl Sub<other_type> for tuple_type {
    type Output = other_type;

    fn sub(self, other: other_type) -> Self::Output {
        let self_iter = self.value.iter();
        let other_iter = other.value().iter();

        Self::Output {
            value: self_iter.zip(other_iter).map(|(&x, &y)| x - y).collect(),
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
)]
impl Mul<f32> for tuple_type {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            value: self.value.iter().map(|&x| x * rhs).collect(),
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
