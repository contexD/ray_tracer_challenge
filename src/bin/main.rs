use ray_tracer_challenge::projectile::{Environment, Projectile};
use ray_tracer_challenge::tuples::{Point, Value, Vector};

fn main() {
    let mut p = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.0, 0.0).normalize(),
    );
    let e = Environment::new(Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));

    while p.position.value()[1] > 0.0 {
        p.tick(&e);
        println!("{:?}", p.position.value());
    }
}
