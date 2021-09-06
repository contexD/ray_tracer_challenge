use crate::tuples::{Point, Vector};

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
    pub fn tick(&mut self, env: &Environment) {
        self.position = &self.position + &self.velocity;
        self.velocity = &self.velocity + &env.gravity + &env.wind;
    }
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}
