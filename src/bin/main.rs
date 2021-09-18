use ray_tracer_challenge::projectile::{Environment, Projectile};
use ray_tracer_challenge::tuples::{Point, Value, Vector};

fn main() {
    let mut p = Projectile::new(
        Point::new(vec![0.0, 1.0, 0.0]),
        Vector::new(vec![1.0, 1.0, 0.0]).normalize() * 1.0,
    );
    let e = Environment::new(
        Vector::new(vec![0.0, -0.1, 0.0]),
        Vector::new(vec![-0.01, 0.0, 0.0]),
    );
    let mut count = 0;

    while p.position.value()[1] > 0.0 {
        p.tick(&e);
        count += 1;
        println!("{:?}", p.position.value());
    }
    println!("number of ticks: {}", &count);
}
